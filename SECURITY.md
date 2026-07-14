# Security Policy

## Supported versions

Security fixes are applied to the latest release on the `master` branch and backported to recent tagged releases at maintainers' discretion.

## Reporting a vulnerability

**Please do not open public GitHub issues for security vulnerabilities.**

Email **support@sconts.com** with:

- Description of the issue and impact
- Steps to reproduce
- Affected version(s)
- Any suggested fix (optional)

We aim to acknowledge reports within **72 hours** and will coordinate disclosure after a fix is available.

## OSS security model

The Community Edition (`superd`) does **not** implement API authentication by default. Shipped example configs use `host = "127.0.0.1"` and **`allow_insecure_public_bind = false`**, so the daemon **refuses startup** on a non-loopback bind unless you explicitly opt in or load the **security** plugin.

To bind on `0.0.0.0` or another network-facing address without the security plugin, set `allow_insecure_public_bind = true` and accept that the REST API is open to anyone who can reach the port (OSS only). **Licensed deployments must load the bundled `security` plugin** — startup fails otherwise. For token-based auth and RBAC, use `auth_secret` with the **security** plugin and a valid `[license].key` in `conf/super.toml`.

### Built-in safeguards (OSS)

Super applies defensive defaults even when no plugins are loaded:

| Safeguard | Behaviour |
| :--- | :--- |
| **Bind policy** | Fail-closed on non-loopback unless `allow_insecure_public_bind = true` or `security` plugin auth is active |
| **Log path confinement** | Custom program log paths must stay under `storage.log_dir` |
| **OTA fetch policy** | Remote artifact URLs must use HTTPS; link-local / metadata targets blocked |
| **Health probes** | HTTP(S) URLs only for outbound health checks |
| **Plugin loading** | Only files under `$SUPER_ROOT/plugins/` matching the signed license |
| **Stack includes** | `[include].files` outside `SUPER_ROOT` ignored |
| **Secret display** | API/CLI mask env values whose keys look sensitive |
| **Docs surface** | Swagger UI disabled by default (`enable_docs = false`) |

Full user-facing detail: [Configuration — OSS security defaults](https://super.docs.sconts.com/docs/02-essentials/configuration/#oss-security-defaults-fail-closed).

> **Licensed plugins:** Optional subscription capabilities load as signed plugins with a vendor-supplied `[license].key`. See the [feature matrix](https://super.docs.sconts.com/docs/07-editions/feature-matrix/) and [authentication](https://super.docs.sconts.com/docs/05-advanced-management/authentication/).
