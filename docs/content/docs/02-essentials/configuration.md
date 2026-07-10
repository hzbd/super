---
title: "Configuration"
weight: 2
description: "Understanding the super.toml configuration file format."
---

Super uses TOML (Tom's Obvious, Minimal Language) for configuration. By default, it looks for `super.toml` in the current directory, `/etc/super/`, or `~/.super/`.

## Server Configuration

The `[server]` section controls the `superd` daemon itself.

```toml
[server]
# The IP and port for the API and Web UI
host = "0.0.0.0"
port = 9002

# Graceful shutdown timeout (seconds)
shutdown_timeout = 10

# Flapping detection (e.g., max 5 restarts in 60 seconds)
flapping_window = 60
flapping_threshold = 5

[logging]
# Daemon's own log level (debug, info, warn, error)
log_level = "info"
```

> **OSS security:** Pre-built OSS builds default to `127.0.0.1`. Use `0.0.0.0` only when you intend to expose the API and dashboard — protect the port with a firewall or reverse proxy, or load the **`security` plugin** for built-in auth.

## Program Configuration

You define managed processes using `[[programs]]` blocks. You can have as many as you like.

### Basic Example

```toml
[[programs]]
name = "my-worker"
command = "/usr/local/bin/worker"
args = ["--config", "/etc/worker.conf"]
cwd = "/tmp"
autostart = true
```

### Environment Variables

You can inject environment variables into the process.

```toml
[programs.env]
NODE_ENV = "production"
DB_HOST = "10.0.0.5"
```

> **Note**: Super automatically injects metadata variables like `SUPER_ID`, `SUPER_NAME`, and `SUPER_HOSTNAME` into the child process.

### User & Group

If running as root, you can drop privileges to a specific user.

```toml
[[programs]]
name = "safe-service"
command = "./app"
user = "www-data"
# group = "www-data" # Optional, defaults to user's primary group
```

### Advanced Settings (OSS)

Dependency orchestration in `super.toml`:

```toml
[[programs]]
name = "heavy-job"
command = "./processor"
depends_on = ["database", "redis"]
```

### Plugin-only blocks 💎

The following require **commercial plugins** (`isolation` on Linux for cgroup limits). OSS accepts `resource_limits` in the API schema but does not enforce them without the plugin.

```toml
[[programs]]
name = "heavy-job"
command = "./processor"

[programs.resource_limits]
memory_limit = 536870912  # 512 MB
cpu_quota = 50.0          # 50% of one core
```

### Scheduled tasks

Cron scheduling is built into OSS `superd`. See [Scheduled Tasks](/docs/02-essentials/scheduled-tasks).

```toml
[[programs]]
name = "nightly-backup"
command = "/scripts/backup.sh"
cron = "0 0 2 * * *"   # see Scheduled Tasks doc
```

See [Resource Isolation](/docs/05-advanced-management/resource-isolation) and [Scheduled Tasks](/docs/02-essentials/scheduled-tasks).

### Restart & stop behaviour

Supervisor-compatible restart and stop settings:

```toml
[[programs]]
name = "api-server"
command = "/usr/local/bin/api"
autostart = true
autorestart = "unexpected"   # restart on unexpected exit only
exitcodes = [0]
retry_limit = 3
startsecs = 10               # stable run resets retry counter
stopsecs = 30                # SIGTERM grace before SIGKILL (optional)
priority = 100               # lower = starts earlier on boot
```

For a full list of options, see the [Config Reference](/docs/06-internals/config-reference). Cron scheduling: [Scheduled Tasks](/docs/02-essentials/scheduled-tasks).
