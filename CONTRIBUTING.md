# Contributing to Project Super

Thank you for your interest in contributing to **Project Super** (OSS, MIT).

## Getting started

1. Fork [hzbd/super](https://github.com/hzbd/super) and clone your fork.
2. Install [Rust](https://rust.rust-lang.org/tools/install) (stable).
3. Build: `make build` (`superd` + `super` CLI).
4. Run tests: `cargo test --workspace` and `cargo clippy --workspace --all-targets -- -D warnings`.

Docs site (optional): `make docs-serve` or `cd docs && hugo server -D --disableFastRender` (requires [Hugo Extended](https://gohugo.io/installation/) and the `hextra` submodule). Open **http://localhost:1313/** — do not open `docs/public/` directly; `hugo.yaml` `baseURL` is for production builds (CI overrides it on deploy).

## Pull requests

- Keep changes focused; one logical change per PR.
- Update docs when you change user-visible behaviour or config.
- Ensure CI passes (clippy + tests).
- Write clear commit messages (what and why).

## Scope

This repository is the **open-source core** (`superd`, `super`, MIT). Optional subscription capabilities (API auth, notifications, cgroup limits, dashboard UI) are loaded at **runtime** from signed plugin libraries — they are **not built from this repo** and are out of scope for most OSS PRs.

## OSS vs subscription runtime

Same `superd` binary; subscription unlocks features via config + plugin files:

```
super (this repo, MIT)              subscription delivery (separate)
────────────────────────            ─────────────────────────────────
superd ── verifies ──► [license].key   signed key from your vendor
       ── dlopen ───► plugins/*.so     official plugin libraries
       ── OSS API ──► process control   optional: auth, UI, notify, …
```

**In scope here:** process manager, REST/WS API, plugin host (verify + dlopen + ABIs), `[license]` **verification** only.

**Out of scope here:** plugin implementations, subscription key **issuance**, commercial plugin catalogs, dashboard sources.

Licensed-plugin fields in config and API are documented with a 💎 marker; see [Feature matrix](/docs/07-editions/feature-matrix/).

## Questions

Open a [GitHub Discussion](https://github.com/hzbd/super/discussions) or an issue for design questions before large refactors.
