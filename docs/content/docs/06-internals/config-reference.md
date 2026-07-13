---
title: "Config Reference"
weight: 3
description: "Complete schema for super.toml."
---

## Edition legend

| Mark | Meaning |
| :--- | :--- |
| **💎 Subscription** | Requires valid `[license].key` in `conf/super.toml` and matching authorized plugin libraries. OSS ignores unknown subscription-only fields. |
| *(no mark)* | Available in OSS (with or without plugins). |

> **Pre-release:** Licensed plugins are documented for the target architecture. **Subscription delivery is not open yet** — do not treat 💎 fields as production-supported for paying customers until maintainers announce GA.

**Licensed-plugin fields in this reference** (quick index):

| Location | Keys / file |
| :--- | :--- |
| Root (`super.toml`) | `auth_secret` 💎 |
| `[license]` | `key` 💎 — cryptographically signed subscription token from your vendor |
| `[[programs]]` | `[programs.resource_limits]` (`cpu_quota`, `memory_limit`) 💎 |
| `conf/notify.toml` *(separate file)* | `[[channels]]` 💎 — see [Event Notifications](/docs/05-advanced-management/event-notifications) |

> **Not licensed-only:** `[webhook]` in `super.toml` is parsed but **not wired** at runtime. `[programs.hooks]` and `[[event_hooks]]` work in OSS.

> **OSS security:** See [Configuration — OSS security defaults](/docs/02-essentials/configuration#oss-security-defaults-fail-closed) for fail-closed bind, log path confinement, and other defensive defaults.

## `[server]`

Global settings for the daemon.

| Key | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `host` | string | `127.0.0.1` | Bind address for API/Web UI. |
| `port` | int | `9002` | Bind port. |
| `allow_insecure_public_bind` | bool | `false` | Explicit opt-in to bind on a non-loopback address without the `security` plugin. OSS **refuses startup** when `host` is not loopback and this is `false`. **Licensed deployments always load `security`** — this flag applies to OSS only. |
| `shutdown_timeout` | int | `10` | Seconds to wait for SIGTERM before SIGKILL during shutdown. |
| `flapping_window` | int | `60` | Time window (seconds) to detect restart loops. |
| `flapping_threshold` | int | `5` | Max restarts allowed within the window. |
| `enable_docs` | bool | `false` | Enable Swagger UI (`/swagger-ui`) when the binary is built with the docs feature. |

## Root keys (Licensed 💎)

Top-level fields in `super.toml` (sibling to `[server]`, not inside it):

| Key | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `auth_secret` 💎 | string | — | **Plugin only** (`security`). Root secret for token bootstrap. See [Authentication](/docs/05-advanced-management/authentication). |

## `[license]` — subscription key (Licensed 💎)

Optional section in `conf/super.toml`. When present and valid, `superd` loads authorized plugins from `plugins/` and **requires the bundled `security` plugin** (`security.so` + `auth_secret`) or refuses startup. See [Licensed deployments require security](/docs/05-advanced-management/authentication#licensed-deployments-require-security).

| Key | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `key` 💎 | string | — | Base64-encoded signed subscription key. Obtain from your subscription vendor. Override: `SUPER_LICENSE` env (same format). |

```toml
[license]
key = "eyJjbGFpbXMiOnsiaXNzdWVkX3RvIjoi..."
```

## `[webhook]` — reserved, not active

`super.toml` accepts an optional `[webhook]` block for historical schema compatibility. **The OSS daemon does not read or use this section at runtime** — setting it has no effect today.

| Key | Type | Description |
| :--- | :--- | :--- |
| `url` | string | Parsed but **not connected** to any notifier. |
| `type` | string | Default `generic`. Ignored at runtime. |

**Use licensed notifications instead:** configure webhooks in a separate `conf/notify.toml` file (`[[channels]]`), not in `super.toml`. Requires the `notify` plugin. See [Event Notifications](/docs/05-advanced-management/event-notifications).

> Do not confuse `[webhook]` in `super.toml` with `type = "webhook"` channels in `notify.toml` — only the latter is functional.

## `[storage]` / `[logging]` / `[child_logging]`

See [Configuration](/docs/02-essentials/configuration) for examples. Keys mirror `ServerConfig` in `common/src/config.rs`.

## `[[programs]]`

You can have multiple program blocks.

> **Field naming**: Keys such as `autostart`, `autorestart`, `exitcodes`, `startsecs`, and `stopsecs` align with [Supervisor](/docs/04-production-scenarios/migrations/vs-supervisor) for migration. Newer keys (`retry_limit`, `health_check`, `depends_on`, …) use snake_case. In TOML, `stopwaitsecs` is accepted as an alias for `stopsecs`.

### Identity & execution

| Key | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `name` | string | — | **Required.** Unique program name. |
| `command` | string | — | **Required.** Path to the executable. |
| `args` | list | `[]` | Command-line arguments. |
| `env` | dict | `{}` | Inline environment variables (`KEY = "VAL"`). |
| `env_file` | string | — | Path to a `.env` file loaded at spawn time. |
| `cwd` | string | — | Working directory. |
| `user` | string | — | Run as this user (requires root). |
| `group` | string | — | Logical group for batch control (e.g. `@backend`). |

### Restart & stop behaviour

`autostart` and `autorestart` are **independent**:

* **`autostart`** — start the program when `superd` boots.
* **`autorestart`** — restart the program **after it exits** (crash recovery).

Example: `autostart = false` with `autorestart = "true"` gives a manually started service that still recovers from crashes.

| Key | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `autostart` | bool | `true` | Start on daemon boot. Cron programs skip boot-time start regardless. |
| `autorestart` | string | `unexpected` | `unexpected` — restart unless exit code is in `exitcodes`; `true` — always restart; `false` — never restart on exit. |
| `exitcodes` | list | `[0]` | Exit codes treated as success when `autorestart = "unexpected"`. |
| `retry_limit` | int | `3` | Max consecutive crash restarts before status becomes `Fatal`. |
| `startsecs` | int | `10` | Seconds of stable uptime before an exit resets the retry counter. |
| `stopsecs` | int | `[server].shutdown_timeout` | Per-program seconds to wait after SIGTERM before SIGKILL. Omit to use `[server].shutdown_timeout` (default `10`). TOML alias: `stopwaitsecs`. |
| `priority` | int | `999` | Boot-time autostart order; lower values start first. |

### Logging

| Key | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `stdout_logfile` | string | `{log_dir}/{uuid}.out` | Custom stdout log path (must resolve under `storage.log_dir`). |
| `stderr_logfile` | string | `{log_dir}/{uuid}.err` | Custom stderr log path (must resolve under `storage.log_dir`). |

### Orchestration

| Key | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `depends_on` | list | `[]` | Program names that must be **Healthy** before this one starts. |
| `cron` | string | — | Cron expression (e.g. `0 0 * * * *`). See [Scheduled Tasks](/docs/02-essentials/scheduled-tasks). |

### `[programs.hooks]`

Per-program lifecycle shell hooks. Full behavior table: [Lifecycle Hooks](/docs/03-orchestration/lifecycle-hooks).

| Key | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `pre_start` | string | — | Run before spawn; non-zero exit aborts start. |
| `post_start` | string | — | Run after PID assigned (async). |
| `pre_stop` | string | — | Run before stop signal (sync). |
| `post_stop` | string | — | Run after process exits (async). |

### `[programs.health_check]`

| Key | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `type` | string | — | **Required.** `tcp`, `http`, or `exec`. |
| `port` | int | — | For `tcp` checks. |
| `url` | string | — | For `http` checks. |
| `command` | string | — | For `exec` checks. |

### `[programs.resource_limits]` 💎

**Commercial only.** Linux cgroups CPU/memory limits; requires the `isolation` plugin on Linux. See [Resource Isolation](/docs/05-advanced-management/resource-isolation).

| Key | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `cpu_quota` 💎 | float | — | CPU quota percentage (`100.0` = one core). |
| `memory_limit` 💎 | int | — | Hard memory limit in bytes. |

## `[[event_hooks]]` *(OSS)*

Global event listeners (local scripts, JSON on stdin). Distinct from licensed `conf/notify.toml` webhooks (`notify` plugin). Full reference: [Event Hooks](/docs/03-orchestration/event-hooks).

| Key | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `command` | string | *(required)* | Shell command (`sh -c`). Receives JSON on stdin. |
| `events` | list | `["*"]` | Event names (`process_fatal`, …) or `"*"`. |
| `programs` | list | `["*"]` | Program names to match, or `"*"`. |
| `async` | bool | `true` | Run hook in background task. |
| `timeout_secs` | int | `30` | Kill hook script after N seconds. |
| `id` | string | — | Optional label for logs. |

## `conf/notify.toml` 💎

**Licensed plugin only** (`notify`). Separate from `super.toml`. Hot-reloadable webhook / IM channels. Schema and presets: [Event Notifications](/docs/05-advanced-management/event-notifications).

## System events & reactions

Super emits [System Events](/docs/03-orchestration/system-events) (`process_fatal`, `process_started`, etc.). Where to configure reactions:

| Mechanism | Config file | Edition | Status |
| :--- | :--- | :--- | :--- |
| Lifecycle hooks | `[[programs]]` → `[programs.hooks]` | OSS | ✅ Active |
| Event hooks | `super.toml` → `[[event_hooks]]` | OSS | ✅ [Event Hooks](/docs/03-orchestration/event-hooks) |
| Webhook notifications | `conf/notify.toml` | 💎 Licensed (`notify`) | ✅ [Event Notifications](/docs/05-advanced-management/event-notifications) |
| `[webhook]` in `super.toml` | `[webhook]` | — | ⚠️ Parsed only — not wired |
