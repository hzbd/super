---
title: "Event Hooks"
weight: 5
description: "Run local scripts on system events with structured JSON on stdin."
---

Event hooks let you react to [System Events](/docs/03-orchestration/system-events) by running shell commands on the **same machine** as `superd`. This is the OSS equivalent of Supervisor's `[eventlistener]` — distinct from licensed [Event Notifications](/docs/05-advanced-management/event-notifications) (`notify` plugin), which POST to external IM/webhook URLs.

## Configuration

Define global hooks in `super.toml`:

```toml
[[event_hooks]]
id = "archive-on-fatal"
command = "/opt/super/archive.sh"
events = ["process_fatal"]
programs = ["*"]          # default: all programs
async = true              # default: true
timeout_secs = 30         # default: 30

[[event_hooks]]
command = "python3 /etc/super/handler.py"
events = ["process_backoff", "process_fatal"]
programs = ["api-server", "worker"]
async = false             # run sequentially (still non-blocking for the manager)
```

Reload hooks without restarting programs:

```bash
super reload    # re-reads super.toml, including [[event_hooks]]
```

## JSON payload (stdin)

Each matching hook receives one JSON object on **stdin**:

```json
{
  "event": "process_fatal",
  "timestamp": "2026-07-06T16:16:00Z",
  "hostname": "prod-1",
  "version": "1.1.9",
  "program": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "web-server",
    "pid": 1234,
    "uptime_secs": 502
  },
  "payload": {
    "exit_code": 137,
    "msg": "Stopped after 3 retries.",
    "log_tail": null
  }
}
```

`system_startup` / `system_shutdown` events omit the `program` field.

## Environment variables

In addition to stdin JSON, hooks receive:

| Variable | When set |
| :--- | :--- |
| `SUPER_EVENT` | Always |
| `SUPER_HOSTNAME` | Always |
| `SUPER_ID` | Program events |
| `SUPER_NAME` | Program events |
| `SUPER_PID` | When PID is known |
| `SUPER_EXIT_CODE` | Fatal / backoff with exit code |
| `SUPER_UPTIME_SECS` | Fatal / backoff / recovered |

## Behavior

* Commands run via `sh -c` (pipes and redirects work).
* Hooks **never block** process management — failures are logged only.
* `async = true` (default): each hook runs in its own task.
* `async = false`: hooks for the same event run one after another in a background task.
* Non-zero exit or timeout → warning log; no impact on managed processes.

## OSS vs licensed notifications

| | Event hooks (OSS) | Notifications (Licensed 💎) |
| :--- | :--- | :--- |
| Config | `super.toml` → `[[event_hooks]]` | `conf/notify.toml` (`notify` plugin) |
| Execution | Local script | HTTP to Slack / 钉钉 / etc. |
| Data | JSON stdin + env | Rich envelope + IM templates |

You can use both: licensed notify for on-call alerts, event hooks for local automation (archiving, systemd triggers, etc.).

## Related

* [System Events](/docs/03-orchestration/system-events) — full event catalog
* [Lifecycle Hooks](/docs/03-orchestration/lifecycle-hooks) — per-program start/stop scripts
* [Config Reference](/docs/06-internals/config-reference#event_hooks-oss)
