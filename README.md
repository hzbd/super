# 🦸 Project Super (Community Edition)

**The API-First, Lightweight Process Orchestrator for the Edge.**

Super is a modern replacement for tools like [Supervisor](https://supervisord.org/) or [PM2](https://pm2.keymetrics.io/), built with **Rust**. It is designed for edge computing, IoT devices, and high-performance servers.

> **Public trial (not for production yet)**
>
> Super is under intensive testing — **do not run it in production**. We are looking for early feedback.
>
> - **OSS core** (`superd` + `super`) is free under MIT — install and try anytime.
> - **Super Pro plugins** (Dashboard UI, API auth/RBAC/audit, notifications, Linux cgroup isolation) are available with a **free 1-year license** on request. No payment required during this trial phase.
>
> **Request a free Pro trial:** open a [GitHub Issue](https://github.com/hzbd/super/issues/new?template=pro-trial.yml) (use the **Pro trial request** template). Include a contact email — we will send the license key and plugin package to that address.

> **Documentation:** [https://super.docs.sconts.com/docs/](https://super.docs.sconts.com/docs/)

## Core Features

* **Single binary** — Rust `superd` process manager; TOML or REST config; CLI and HTTP API (Dashboard via optional `ui` plugin)
* **Declarative orchestration** — stacks, dependencies, health checks
* **Lifecycle hooks** — `pre_start`, `post_start`, `post_stop`, and global event hooks
* **Observability** — WebSocket logs, historical logs API, system metrics
* **Auto-recovery** — Supervisor-compatible `autorestart`, `exitcodes`, `startsecs`

Licensed under the **[MIT License](LICENSE)**. Optional **licensed plugins** (`.so` / `.dylib` under `$SUPER_ROOT/plugins/`) add API auth, RBAC, notifications, and cgroup limits — same `superd` binary, no separate commercial build. Compare editions in the [feature matrix](https://super.docs.sconts.com/docs/07-editions/feature-matrix/).

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
| Getting started | [Docs](https://super.docs.sconts.com/docs/01-getting-started/) |
| Configuration | [Config reference](https://super.docs.sconts.com/docs/06-internals/config-reference/) |
| API | [API reference](https://super.docs.sconts.com/docs/06-internals/api-reference/) |
| Changelog | [v1.2.1](https://super.docs.sconts.com/docs/08-changelog/) |
| Editions / Pro plugins | [Feature matrix](https://super.docs.sconts.com/docs/07-editions/feature-matrix/) |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Security issues: [SECURITY.md](SECURITY.md).
