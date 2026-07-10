---
title: "Changelog"
weight: 8
description: "All notable changes to Project Super will be documented in this file."
---

All notable changes to **Project Super** will be documented in this page.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Changed
- Docker Hub image CI publishes **`linux/amd64` only** (removed arm64 manifest from workflow to speed up builds).
- **Cron scheduled tasks** are available in the open-source `superd` (no longer Premium-only).
- Open-source edition license changed from **GPL-3.0** to **MIT**.

---

## [1.1.9] - 2026-07-08

### Added
- GitHub Releases **multi-platform binaries** (Linux amd64/arm64, macOS Intel/ARM, FreeBSD) with README archives and `SHA256SUMS`.
- Docker image **multi-arch** publish (`linux/amd64`, `linux/arm64`).
- `gh-pages` branch README (auto-deployed with documentation).

### Changed
- Docker image: **Debian 13 (trixie)** build stages and **distroless `cc-debian13`** runtime.
- Release CI uses native `ubuntu-24.04-arm` for Linux ARM64 builds.

### Notes
- **Windows** pre-built binaries are not published; use Docker or build on Unix-like systems.

### Fixed
- FreeBSD release packaging (version passed into VM).
- CLI `check.rs` explicit `Vec<String>` types.

---

## [1.1.8] - 2026-07-07

### Added
- Official Docker image **`containerpi/super`** with default config under `dockerbuild/conf/`.
- GitHub Actions workflow to build and push the Docker image.
- Documentation homepage with OSS capabilities, licensed plugin features, and API example.

### Changed
- Docker image namespace from `hzbd/super` to `containerpi/super`.
- Installation docs, README, and `make docker` target for `dockerbuild/Dockerfile`.

### Fixed
- Dashboard `ProcessList.vue` syntax error breaking `vue-tsc` build.
- Doc screenshot paths for GitHub Pages (`/super/images/...`).

---

## [1.1.7] - 2026-07-07

### Added
- Event hooks — run scripts on [system events](/docs/03-orchestration/system-events).
- `post_stop` lifecycle hook.
- Supervisor-compatible fields: `stopsecs`, `priority`, log file paths, `autorestart` / `exitcodes` / `startsecs`.
- Historical logs API and `super logs --tail`.
- OTA updates via API and `super update --artifact-*`.
- System stats API and dashboard metrics panel.

### Changed
- OSS API no longer uses `auth_secret`; bind to `127.0.0.1` or use a firewall for exposure control.
- Documentation updates across config, API, and feature matrix.

### Fixed
- Historical logs API now reads from the correct log directory when `[storage]` is omitted.
