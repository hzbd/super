---
title: "Lifecycle Hooks"
weight: 3
description: "Inject custom scripts at various stages of the process lifecycle."
---

Lifecycle hooks run **shell commands** at fixed points in a **single program's** start/stop pipeline. They are configured **per program**, not globally.

For reacting to **system-wide events** (crashes, recovery, daemon startup), see [System Events](/docs/03-orchestration/system-events) — those use a different configuration model (`notify.toml` with the `notify` plugin; `[[event_hooks]]` in OSS).

## Hook catalog

| Hook | When it runs | Blocks start/stop? | Execution | Implemented |
| :--- | :--- | :--- | :--- | :--- |
| `pre_start` | After extension `before_start`, **before** `spawn` | **Yes** — non-zero exit aborts start; program → `Fatal` | Awaited (sync) | ✅ |
| `post_start` | After PID assigned, extension `after_start`, and `process_started` event | No | Async (`tokio::spawn`) | ✅ |
| `pre_stop` | After user/API stop requested, **before** `SIGTERM` / `SIGKILL` | Delays signal until hook returns | Awaited (sync) | ✅ |
| `post_stop` | After process has exited | No | Async (`tokio::spawn`) | ✅ |

### Execution details

* Commands run via `sh -c`, so pipes, redirects, and `$()` work.
* Empty or whitespace-only commands are skipped.
* Hook stdout/stderr inherit `superd`'s logging stream.
* `pre_start` failure emits a `process_fatal` system event with message `"Pre-start hook failed"`.

## Configuration

Hooks are stored on each program — in `super.toml`, stack JSON, or via the API / Dashboard.

```toml
[[programs]]
name = "my-app"
command = "./app"

[programs.hooks]
pre_start = "mkdir -p /tmp/app-data && chmod 700 /tmp/app-data"
post_start = "curl -X POST http://consul:8500/register -d '...'"
pre_stop = "curl -X POST http://consul:8500/deregister -d '...'"
post_stop = "if [ \"$SUPER_EXIT_CODE\" != \"0\" ]; then aws s3 cp logs/app.err s3://archive/; fi"
```

Equivalent in stack JSON:

```json
{
  "name": "my-app",
  "command": "./app",
  "hooks": {
    "pre_start": "echo checking...",
    "post_start": "echo registered"
  }
}
```

### Where this config lives

| Source | Path / API | Persisted to |
| :--- | :--- | :--- |
| TOML file | `[[programs]]` → `[programs.hooks]` | Loaded into registry on daemon start |
| Stack apply | `POST /api/stack/apply` with `"hooks": { ... }` | `snapshot.json` |
| CLI | `super add` / `super update --pre-start "..."` | `snapshot.json` |
| Dashboard | Program create/edit form | `snapshot.json` |

Hooks are **not** defined in `super.toml` at the global level — only under each `[[programs]]` block (or via API/stack).

## Environment variables

Hook scripts receive context via environment variables (in addition to the program's `env` / `env_file`):

| Variable | `pre_start` | `post_start` | `pre_stop` | `post_stop` |
| :--- | :---: | :---: | :---: | :---: |
| `SUPER_ID` | ✅ | ✅ | ✅ | ✅ |
| `SUPER_NAME` | ✅ | ✅ | ✅ | ✅ |
| `SUPER_HOSTNAME` | ✅ | ✅ | ✅ | ✅ |
| `SUPER_GROUP` | ✅ if set | ✅ if set | ✅ if set | ✅ if set |
| `SUPER_PID` | — | ✅ | ✅ | ✅ |
| `SUPER_EXIT_CODE` | — | — | — | ✅ |
| `SUPER_UPTIME_SECS` | — | — | — | ✅ |

The managed **child process** also receives `SUPER_ID`, `SUPER_NAME`, `SUPER_HOSTNAME`, and optional `SUPER_GROUP` at spawn time (not the full hook set).

## Lifecycle vs system events

| | Lifecycle hooks | System events |
| :--- | :--- | :--- |
| **Purpose** | Setup/teardown around one program's start/stop | Observe cluster-wide incidents |
| **Config** | Per program `[programs.hooks]` | Global (`[[event_hooks]]` in OSS; `notify.toml` with `notify` plugin) |
| **Data to script** | Environment variables | JSON on stdin (OSS event hooks) or HTTP payload (licensed notify) |
| **Examples** | mkdir, Consul register, drain flag | Slack alert, archive logs on crash |

## Related

* [System Events](/docs/03-orchestration/system-events) — full event catalog
* [Config Reference — programs.hooks](/docs/06-internals/config-reference#programshooks)
* [Custom Extensions](/docs/04-production-scenarios/extensibility/custom-extensions) — Rust `Extension` trait and licensed plugins (cgroups, notify, audit)
