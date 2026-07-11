---
title: "OSS Repository Boundary"
weight: 2
description: "What belongs in the public super repo vs subscription runtime."
---

This page is for **contributors and doc authors** working on the public [hzbd/super](https://github.com/hzbd/super) repository.

## One binary, optional plugins

| Layer | In this repo (MIT) | Outside this repo |
| :--- | :--- | :--- |
| **Daemon & CLI** | `superd`, `super` | — |
| **Process API** | REST, WebSocket, metrics | — |
| **Plugin host** | Verify signed `[license].key`, dlopen `plugins/*` | Plugin library **binaries** |
| **ABIs** | `plugin_abi`, `plugin_http_abi`, `plugin_ui_abi` | Plugin **implementations** |
| **License** | Ed25519 **verification**, claims schema | Key **signing**, vendor delivery |

There is no separate “premium” build of `superd`. Subscription features unlock when a valid key and matching plugin files are present at runtime.

## Safe to document publicly

- OSS behaviour without plugins (localhost API, no auth by default).
- Config fields marked 💎 and what they do **when a subscription is active**.
- Runtime routes such as `GET /api/system/license` (populated only when a key verifies).
- Generic deployment: paste vendor key → copy delivered plugin libraries → restart.

## Do not document in this repo

- Internal signing tools, private repositories, or build pipelines for plugin/dashboard artifacts.
- Default issuance policy (version headroom, retain-after-expiry defaults, plugin SKU lists used at signing time).
- File paths or directory layouts of non-public source trees.

Use vendor-neutral wording: **“subscription delivery package”**, **“your vendor”**, **“authorized plugin libraries”**.

## Code layout (this repo)

```
common/src/license/     claims + verify only (runtime)
core/src/plugin/        discovery, dlopen, HTTP/UI bridges
common/src/plugin_*     stable C ABIs for .so/.dylib plugins
```

Do not add plugin catalogs, feature-to-plugin maps, or signing helpers to `common/`.

## Contributor checklist

Before opening a PR, ask:

1. Does this change require a private tree or signing key? → It belongs outside this repo.
2. Does a comment or doc link name an internal tool or private checkout path? → Rephrase.
3. Is the 💎 marker used only for fields that need a verified subscription at runtime? → Keep docs accurate.

See also [CONTRIBUTING.md](https://github.com/hzbd/super/blob/master/CONTRIBUTING.md) and the [Feature matrix](/docs/07-editions/feature-matrix/).
