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

The Community Edition (`superd`) does **not** implement API authentication by default. Bind to `127.0.0.1` or protect the API port with a firewall. For token-based auth and RBAC, load the **`security`** licensed plugin (`plugins/security.so` + valid `[license].key` in `conf/super.toml`).

> **Licensed plugins (v1.2.0):** Pre-release — not offered for subscription delivery yet. Do not expose plugin-gated deployments as production-ready without maintainer sign-off.
