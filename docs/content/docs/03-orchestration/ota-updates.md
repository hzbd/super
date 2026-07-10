---
title: "Atomic OTA Updates"
weight: 4
description: "Perform fail-safe, transactional updates for your binaries."
---

Updating software on remote edge devices or production servers is risky. A partial download or a corrupted binary can leave the system in an unrecoverable state ("bricked").

Super solves this with **Transactional OTA (Over-The-Air) Updates**.

## The Transactional Flow

When you trigger an update, Super acts like a database transaction: **All or Nothing**.

1.  **Download**: The new binary is downloaded to a staging file (e.g., `app.new`).
2.  **Verify**: Checksum (SHA256) is verified.
3.  **Backup**: The current running binary is hard-linked to a backup (e.g., `app.bak`).
4.  **WAL**: The "Upgrade In-Progress" state is written to disk (Write-Ahead Log).
5.  **Swap**: The new binary replaces the old one atomically.
6.  **Restart**: The process is restarted.
7.  **Validate**: Super waits for the `health_check` to pass.
    *   ✅ **Success**: The backup is removed. Transaction committed.
    *   ❌ **Failure**: The process crashes or fails health checks. **Rollback** is triggered. The backup is restored, and the old version is restarted.

## Triggering an Update

Provide a new `artifact` block with a **different `checksum`** than the one already stored. Super compares checksums; if unchanged, config is saved but **no OTA download** runs.

### Via API (recommended for CI/CD)

Use **`PUT /api/programs/{id}`** with the program UUID. Full reference: [API Reference — Update Program](/docs/06-internals/api-reference#update-program).

```bash
# 1. Resolve UUID by name
PROGRAM_ID=$(curl -s http://127.0.0.1:9002/api/programs \
  | jq -r '.[] | select(.name=="my-app") | .id')

# 2. Trigger OTA
curl -X PUT "http://127.0.0.1:9002/api/programs/${PROGRAM_ID}" \
  -H "Content-Type: application/json" \
  -d '{
    "artifact": {
      "source": "https://example.com/builds/v2.0.0/app-linux-amd64",
      "checksum": "a1b2c3d4e5f6789abcdef0123456789abcdef0123456789abcdef0123456789",
      "destination": "/usr/local/bin/my-app",
      "extract": false,
      "restart_policy": "immediate"
    }
  }'
```

With the `security` plugin: add `-H "Authorization: Bearer <token>"`.

### Via Stack (declarative, multi-service)

```bash
curl -X PUT http://127.0.0.1:9002/api/stack \
  -H "Content-Type: application/json" \
  -d '{
    "prune": false,
    "services": [{
      "name": "my-app",
      "command": "/usr/local/bin/my-app",
      "artifact": {
        "source": "https://example.com/builds/v2.0.0/app-linux-amd64",
        "checksum": "a1b2c3d4e5f6789abcdef0123456789abcdef0123456789abcdef0123456789",
        "destination": "/usr/local/bin/my-app",
        "extract": false,
        "restart_policy": "immediate"
      }
    }]
  }'
```

### Via CLI

```bash
super update my-app \
  --artifact-url "https://example.com/builds/v2.0.0/app-linux-amd64" \
  --artifact-sha256 "a1b2c3d4e5f6789abcdef0123456789abcdef0123456789abcdef0123456789"
```

If the program already has an `artifact.destination`, you can omit `--artifact-destination`. Otherwise pass it explicitly:

```bash
super update my-app \
  --artifact-url "https://example.com/builds/v2.0.0/app-linux-amd64" \
  --artifact-sha256 "a1b2c3..." \
  --artifact-destination "/usr/local/bin/my-app"
```

For non-OTA config changes:

```bash
super update my-app --command /usr/local/bin/my-app-v2
super restart my-app    # required to run the new command
```

## Artifact schema

| Field | Required | Description |
| :--- | :--- | :--- |
| `source` | Yes | HTTPS (or HTTP) URL to download |
| `checksum` | Yes | SHA256 hex of the artifact |
| `destination` | Yes | Absolute path of the binary on disk |
| `extract` | Yes | `false` for a single binary; `true` for archives |
| `restart_policy` | Yes | `"immediate"` — swap then restart (primary supported policy) |

## Why this matters

*   **No "Half-Downloaded" States**: The running binary is never touched until the new one is fully downloaded and verified.
*   **Automatic Recovery**: If the new version has a segmentation fault or a configuration error, Super restores the previous working version automatically. No manual intervention required.

See also [Fail-Safe OTA](/docs/04-production-scenarios/delivery/fail-safe-ota).
