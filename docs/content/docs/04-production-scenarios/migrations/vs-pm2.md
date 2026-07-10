---
title: "vs PM2"
weight: 2
description: "Escape the Node.js runtime tax. Managing binaries with minimal overhead."
---

[PM2](https://pm2.keymetrics.io/) is an excellent process manager for the Node.js ecosystem. However, it is often misused as a general-purpose supervisor for Go, Python, or Java applications.

When used outside of Node.js, PM2 imposes a heavy **"Runtime Tax"**. Project Super offers a lighter, faster alternative.

## 1. The Memory Overhead

PM2 is written in JavaScript and runs on top of Node.js. This means even if you are managing a tiny 2MB Go binary, you must run a Node.js virtual machine (the PM2 daemon) in the background.

**Benchmark: Idle Daemon Memory Usage**

| Process Manager | Memory Footprint (RSS) | Overhead |
| :--- | :--- | :--- |
| **PM2 (Node.js)** | ~40 MB - 100 MB | High |
| **Project Super (Rust)** | **~3 MB - 8 MB** | **Minimal** |

For memory-constrained environments (like t3.micro instances, Raspberry Pis, or high-density containers), running PM2 is wasteful.

## 2. Resource Limits (Cgroups)

PM2 generally relies on the OS or Docker to handle resource limits. It does not have native support for strictly limiting a child process's CPU or Memory usage via Linux Cgroups.

**Super** integrates natively with Linux Cgroups v2:

```toml
# super.toml
[programs.resource_limits]
memory_limit = 268435456 # 256MB
cpu_quota = 25.0         # 0.25 Core
```

If your worker process leaks memory, Super's Cgroup enforcement will kill it before it crashes the entire server. PM2 cannot do this directly.

## 3. Language Agnostic

PM2 treats non-Node applications as "fork mode" citizens. Advanced features like "cluster mode" (load balancing) only work for Node.js scripts.

**Super** treats **all** binaries equally. Whether it's a Rust binary, a Python script, or a Java JAR, they all get:
*   Unified Logging
*   Health Checks
*   Graceful Shutdown
*   Dependency Orchestration

## 4. Log Rotation

PM2 requires installing an extra module (`pm2-logrotate`) to handle log rotation.

**Super** has [Log Rotation](/docs/02-essentials/logging) built into the core. You don't need to install plugins or manage external dependencies to keep your disk from filling up.

## Summary

*   **Stick with PM2** if you are running a pure Node.js stack and need the specific "Cluster Mode" for zero-downtime Node.js reloads.
*   **Switch to Super** if you are running Go, Rust, Python, Java, or a mix of languages, and want to save ~50MB of RAM per instance while gaining better isolation features.
