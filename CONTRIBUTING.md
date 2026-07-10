# Contributing to Project Super

Thank you for your interest in contributing to **Project Super** (OSS, MIT).

## Getting started

1. Fork [hzbd/super](https://github.com/hzbd/super) and clone your fork.
2. Install [Rust](https://rust.rust-lang.org/tools/install) (stable) and [Node.js](https://nodejs.org/) (for the dashboard).
3. Build: `make build` (frontend + `superd` + `super` CLI).
4. Run tests: `cargo test --workspace` and `cargo clippy --workspace --all-targets -- -D warnings`.

Docs site (optional): `make docs-serve` or `cd docs && hugo server -D --disableFastRender` (requires [Hugo Extended](https://gohugo.io/installation/) and the `hextra` submodule). Open **http://localhost:1313/** — do not open `docs/public/` directly; `hugo.yaml` `baseURL` is for production builds (CI overrides it on deploy).

## Pull requests

- Keep changes focused; one logical change per PR.
- Update docs when you change user-visible behaviour or config.
- Ensure CI passes (clippy + tests).
- Write clear commit messages (what and why).

## Scope

This repository is the **open-source core** (`superd`, `super`, MIT). Commercial capabilities (auth, license verification, notify, audit, cgroups enforcement) ship as **optional runtime plugins** in a separate product repository and are out of scope for most OSS PRs.

## Questions

Open a [GitHub Discussion](https://github.com/hzbd/super/discussions) or an issue for design questions before large refactors.
