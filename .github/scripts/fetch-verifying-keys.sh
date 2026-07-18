#!/usr/bin/env bash
# Fetch Super Pro verifying keyring from Manager into common/keys/ for this build.
#
# Env:
#   MANAGER_BASE   — e.g. https://manager.example.com (default http://127.0.0.1:8787)
#   MANAGER_TOKEN  — Bearer token with products.read
#   PRODUCT_ID     — default super-pro
#   REQUIRE_MANAGER_KEYRING — if 1/true, fail when token missing or fetch fails
#
# Behavior:
#   - Decodes each entries[].public_key_b64 → common/keys/{product}.{kid}.public.key
#   - Replaces prior {product}.*.public.key for that product (exact Manager snapshot)
#   - Leaves common/keys/public.key alone (historical kid v1 embed, if present)
#   - common/build.rs embeds every key file into PUBLIC_KEY_RING at compile time
#
# GitHub Actions: set repository secrets MANAGER_BASE + MANAGER_TOKEN; optional
# repository variable REQUIRE_MANAGER_KEYRING=1 to make release builds require Manager.

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
OSS_KEYS="${OSS_KEYS:-$ROOT/common/keys}"
PRODUCT_ID="${PRODUCT_ID:-super-pro}"
REQUIRE="${REQUIRE_MANAGER_KEYRING:-0}"

require_on() {
  case "$(printf '%s' "$REQUIRE" | tr '[:upper:]' '[:lower:]')" in
    1|true|yes|on) return 0 ;;
    *) return 1 ;;
  esac
}

fail_or_skip() {
  local msg="$1"
  if require_on; then
    echo "ERROR: $msg (REQUIRE_MANAGER_KEYRING is set)" >&2
    exit 1
  fi
  echo "NOTICE: $msg — using committed keys under common/keys/"
  exit 0
}

token="${MANAGER_TOKEN:-}"
if [[ -z "$token" ]]; then
  fail_or_skip "MANAGER_TOKEN is not set"
fi

base="${MANAGER_BASE:-http://127.0.0.1:8787}"
base="${base%/}"
mkdir -p "$OSS_KEYS"

echo "==> GET $base/v1/products/${PRODUCT_ID}/public-keyring"
json=$(curl -fsS \
  -H "Authorization: Bearer $token" \
  -H "Accept: application/json" \
  "$base/v1/products/${PRODUCT_ID}/public-keyring") || {
  fail_or_skip "Manager keyring request failed"
}

PRODUCT_ID="$PRODUCT_ID" OSS_KEYS="$OSS_KEYS" python3 - "$json" <<'PY'
import base64, json, os, sys
from pathlib import Path

def sanitize(raw: str) -> str:
    s = "".join(c if (c.isalnum() or c in "-_.") else "_" for c in raw.strip())
    return s or "_"

data = json.loads(sys.argv[1])
product_id = os.environ["PRODUCT_ID"]
oss = Path(os.environ["OSS_KEYS"])
entries = data.get("entries") or []
if not entries:
    sys.exit("ERROR: keyring has no entries")

# Drop previous named public keys for this product so the tree matches Manager.
prefix = f"{sanitize(product_id)}."
for path in sorted(oss.glob("*.public.key")):
    name = path.name
    if name.startswith(prefix) and name.endswith(".public.key"):
        path.unlink()
        print(f"  removed stale {name}")

written = 0
for e in entries:
    kid = (e.get("kid") or "").strip()
    b64 = (e.get("public_key_b64") or "").strip()
    if not kid or not b64:
        continue
    raw = base64.b64decode(b64)
    if len(raw) != 32:
        sys.exit(f"ERROR: kid={kid} decoded to {len(raw)} bytes (expected 32)")
    stem = f"{sanitize(product_id)}.{sanitize(kid)}"
    out = oss / f"{stem}.public.key"
    out.write_bytes(raw)
    active = " active" if e.get("active") else ""
    print(f"  wrote {out.name} (32 bytes) kid={kid}{active}")
    written += 1

if written == 0:
    sys.exit("ERROR: no keyring entries written")

v1 = oss / "public.key"
if v1.is_file():
    print(f"  kept {v1.name} (historical v1 embed, {v1.stat().st_size} bytes)")
print(f"==> {written} verifying key(s) ready under {oss}")
PY
