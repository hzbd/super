---
title: "Authentication"
weight: 1
description: "Securing the Daemon with Access Tokens."
---

The **default OSS deployment has no API authentication**. By default, `superd` binds to loopback and **refuses to start** on a non-loopback address unless you explicitly set `allow_insecure_public_bind = true` in `[server]` or load the optional **`security` plugin** for token-based auth.

> **Pre-release:** Licensed plugins are **not ready for production or customer delivery**. The steps below are for development and integration testing only.

OSS deployments without a valid `[license].key` have no API auth; public bind requires explicit opt-in via `allow_insecure_public_bind` as described above.

## Licensed deployments require `security`

**Every subscription includes the `security` plugin at no extra charge.** If `[license].key` verifies successfully, `superd` **refuses to start** unless:

1. **`security` is listed in the signed license claims** (re-issue legacy keys that omit it).
2. **`security.so` / `security.dylib` loads successfully** from `$SUPER_ROOT/plugins/`.
3. **`auth_secret` is set** in `conf/super.toml` (root secret for token bootstrap).
4. **HTTP auth middleware is active** (the security plugin exports `authenticate`).

Other licensed plugins (`ui`, `notify`, `isolation`, …) load only after these checks pass. OSS deployments (no valid license) are unchanged.

| Mode | API auth | Startup if `security` missing |
| :--- | :--- | :--- |
| OSS | ❌ Open (loopback-first) | N/A — runs without plugins |
| **Licensed** | ✅ Required (via `security`) | **Hard fail** |

> **Legacy keys** without `security` in claims must be re-issued. **Partial installs** (license OK, `ui.so` present, `security.so` missing) also fail fast with an actionable error.

## Enabling Authentication (Subscription)

1. Add a valid `[license].key` in `conf/super.toml` (must authorize `security` — included with every subscription).
2. Install **`security.so`** from your subscription delivery package into `$SUPER_ROOT/plugins/` (required for startup).
3. Set `auth_secret` in `super.toml` (required for startup):

```toml
# super.toml (subscription)
auth_secret = "my-super-secure-root-password"
```

Once the `security` plugin is active:

1. All API requests require an `Authorization: Bearer <token>` header (except health/docs whitelist).
2. The Web UI prompts for authentication when `auth_required` is injected.

## Bootstrap with Root Secret

Use the root `auth_secret` as a Bearer token to create scoped API tokens:

```bash
# Create an operator token
curl -X POST http://127.0.0.1:9002/api/auth/tokens \
  -H "Authorization: Bearer my-super-secure-root-password" \
  -H "Content-Type: application/json" \
  -d '{"name":"ci-bot","role":"operator"}'
```

Use the returned `sk-...` token for subsequent API calls.

> **Without the security plugin**: OSS `superd` has no `/api/auth/tokens` route. `super login` will fail with 404 until the plugin is loaded.

## Managing Tokens (HTTP API)

### List Tokens

```bash
curl -H "Authorization: Bearer <token>" http://127.0.0.1:9002/api/auth/tokens
```

### Revoke a Token

```bash
curl -X DELETE -H "Authorization: Bearer <admin-token>" \
  http://127.0.0.1:9002/api/auth/tokens/<id>
```

## Roles

| Role | Permissions |
|------|-------------|
| **Viewer** | Read-only (list, info, logs) |
| **Operator** | Start/stop/restart, create/update programs |
| **Admin** | Full access including token management |
