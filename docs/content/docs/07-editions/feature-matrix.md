---
title: "Feature Matrix"
weight: 1
description: "OSS core vs optional licensed plugins."
---

| Feature | OSS (no plugins) | With licensed plugins |
| :--- | :---: | :---: |
| **Core Process Management** | ✅ | ✅ |
| **Dependency Orchestration** | ✅ | ✅ |
| **Atomic OTA Updates** | ✅ | ✅ |
| **Health Checks (TCP/HTTP)** | ✅ | ✅ |
| **Web UI (Dashboard)** | ✅ <br>(no auth) | ✅<br> (auth with `security`) |
| **Log Rotation & Streaming** | ✅ | ✅ |
| **Prometheus Metrics** | ✅ <br>(basic) | ✅ <br>(+ plugin metrics) |
| **Historical Logs API** | ✅ | ✅ |
| **System Stats API** | ✅ | ✅ |
| **Event Hooks** (`[[event_hooks]]`) | ✅ | ✅ |
| **Cron Scheduled Tasks** | ✅ | ✅ |
| **Linux Cgroups Isolation** | ❌ | ✅ (`isolation` plugin) |
| **RBAC (User Roles)** | ❌ | ✅ (`security` plugin) |
| **Audit Logging** | ❌ | ✅ (`security` plugin) |
| **Webhook Notifications** | ❌ | ✅ (`notify` plugin) |
| **License** | MIT (no plugins) | Commercial plugin license |

Same **`superd`** and **`super`** binaries for both columns — drop `plugins/*.so` + `[license].key` in `conf/super.toml` to enable the right-hand column.

## Which setup do I need?

### OSS only (no plugins)
*   Personal projects, homelab, or local development.
*   Trusted private network (VPN/VPC) with firewall in front of the API.
*   No strict per-process CPU/memory enforcement.

### Licensed plugins
*   **PaaS** or shared hosting with cgroup isolation (`isolation`).
*   **Webhook notifications** for on-call (`notify`).
*   Regulated environments needing **audit logs** (`security`).
*   Exposing API/Dashboard beyond localhost with **token authentication** (`security`).
