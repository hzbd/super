# 🦸 Project Super (Community Edition)

**The API-First, Lightweight Process Orchestrator for the Edge.**

Super is a modern replacement for tools like [Supervisor](https://supervisord.org/) or [PM2](https://pm2.keymetrics.io/), built with **Rust**. It is designed for edge computing, IoT devices, and high-performance servers.

> **⚠️ Not for production use yet**
>
> Project Super is still under intensive testing. **Please do not run it in production environments** for now. We are actively hardening stability, edge cases, and operational behavior — production-grade reliability is the goal before we recommend real-world deployments.

> **Documentation:** [http://super.docs.sconts.com/docs/](http://super.docs.sconts.com/docs/)

## Core Features

* **Single binary** — Rust `superd` process manager; TOML or REST config; CLI, web UI, and HTTP API
* **Declarative orchestration** — stacks, dependencies, health checks
* **Lifecycle hooks** — `pre_start`, `post_start`, `post_stop`, and global event hooks
* **Observability** — WebSocket logs, historical logs API, system metrics
* **Auto-recovery** — Supervisor-compatible `autorestart`, `exitcodes`, `startsecs`

Licensed under the **[MIT License](LICENSE)**. Optional **licensed plugins** (`.so` / `.dylib` under `$SUPER_ROOT/plugins/`) add API auth, RBAC, notifications, and cgroup limits — same `superd` binary, no separate commercial build.

> **Licensed plugins (v1.2.0):** The runtime plugin model is in **pre-release** and **not available for subscription delivery** yet. OSS `superd` without plugins remains the supported path for self-hosted use. See the [changelog](http://super.docs.sconts.com/docs/08-changelog/).

## Quick Start

### Docker

Docker image (`linux/amd64`):

```bash
docker pull containerpi/super:latest
docker run --rm -p 9002:9002 containerpi/super:latest
```

With a custom config directory:

```bash
docker run --rm -p 9002:9002 -v ./dockerbuild/conf:/app/super/conf containerpi/super:latest
```

### From source

```bash
git clone https://github.com/hzbd/super.git && cd super
make build
./target/release/superd
```

### CLI

```bash
super add --name redis --autostart /usr/bin/redis-server
super list
super logs <id> --tail
```

## Documentation

| Topic | Link |
|-------|------|
| Getting started | [Docs](http://super.docs.sconts.com/docs/01-getting-started/) |
| Configuration | [Config reference](http://super.docs.sconts.com/docs/06-internals/config-reference/) |
| API | [API reference](http://super.docs.sconts.com/docs/06-internals/api-reference/) |
| Changelog | [v1.2.0 pre-release](http://super.docs.sconts.com/docs/08-changelog/) |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Security issues: [SECURITY.md](SECURITY.md).
