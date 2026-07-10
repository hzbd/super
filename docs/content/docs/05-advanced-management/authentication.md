---
title: "Authentication"
weight: 1
description: "Securing the Daemon with Access Tokens."
---

The **default OSS deployment has no API authentication** — bind to `127.0.0.1` or firewall the API port. Loading the optional **`security` plugin** adds token-based auth for shared or public-facing deployments.

> **Pre-release:** Licensed plugins are **not ready for production or customer delivery**. The steps below are for development and integration testing only.

Without the `security` plugin loaded, `superd` behaves like OSS-only (no API auth, localhost warning when bound publicly).

## Enabling Authentication (Commercial)

1. Add a valid `[license].key` in `conf/super.toml` that includes the `security` plugin.
2. Build and deploy `plugins/security.dylib` (or `.so` on Linux).
3. Set `auth_secret` in `super.toml` (commercial-only field):

```toml
# super.toml (commercial)
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
