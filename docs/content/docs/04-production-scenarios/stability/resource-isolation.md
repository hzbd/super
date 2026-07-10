---
title: "Resource Isolation"
weight: 3
description: "Stop 'noisy neighbors' from crashing your server with Cgroups."
---

In a multi-service environment, one misbehaving process can consume all RAM or CPU and trigger the kernel **OOM killer** — often taking down unrelated services (`web-server`, `ssh`, etc.).

**Licensed plugins (`isolation`)** address this with **Linux Cgroups v2**: hard memory caps and CPU quotas per program, so a runaway worker is contained without Docker.

### Why it matters in production

| Without limits | With Cgroups |
| :--- | :--- |
| One leak can starve the whole host | OOM kills only the offending cgroup |
| Batch jobs steal CPU from APIs | `cpu_quota` throttles low-priority work |
| Need containers for isolation | Bare-metal isolation via `super.toml` |

Configuration, kernel requirements, and metrics are documented in **[Resource Isolation](/docs/05-advanced-management/resource-isolation)** (requires `isolation` plugin).
