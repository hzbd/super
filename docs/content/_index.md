---
title: ""
description: "What Project Super is, and how to install and operate it."
---


<section class="home-hero">
    <h1>Project Super</h1>
    <p class="hero-lead">A lightweight process orchestrator for edge devices and servers — a modern, API-first alternative to Supervisor or PM2.</p>
    <p><code>superd</code> is a Rust process manager — one binary to deploy. Define programs in TOML or over REST; it handles restarts, startup order, and health checks. Manage via CLI, browser, or HTTP API.</p>
  </section>

  <nav class="home-docnav" aria-label="Quick navigation">
    <ul class="home-docnav-list">
      <li><a href="/docs/">Overview</a></li>
      <li><a href="/docs/01-getting-started/">Getting Started</a></li>
      <li><a href="/docs/02-essentials/">Core Essentials</a></li>
      <li><a href="/docs/03-orchestration/">Orchestration</a></li>
      <li><a href="/docs/05-advanced-management/">Advanced Management</a></li>
      <li><a href="/docs/06-internals/api-reference/">API Reference</a></li>
    </ul>
  </nav>

  <section class="home-section">
    <h2 class="home-section-title">Core Capabilities</h2>
    <div class="home-grid home-grid--3">
      <article class="home-feature">
        <h3>Dependency Orchestration</h3>
        <p>Declare <code>depends_on</code> and start order — Super waits for upstream programs to become healthy before starting dependents.</p>
        <a href="/docs/03-orchestration/dependencies">Dependencies →</a>
      </article>
      <article class="home-feature">
        <h3>Atomic OTA Updates</h3>
        <p>Download, verify checksum, swap artifacts, restart, and roll back automatically if health checks fail.</p>
        <a href="/docs/03-orchestration/ota-updates">OTA flow →</a>
      </article>
      <article class="home-feature">
        <h3>Health Checks</h3>
        <p>Scheduled TCP and HTTP probes. Failed checks mark the process unhealthy and block dependents until checks pass.</p>
        <a href="/docs/03-orchestration/health-checks">Health checks →</a>
      </article>
      <article class="home-feature">
        <h3>Lifecycle Hooks</h3>
        <p>Run shell scripts at <code>pre_start</code>, <code>post_start</code>, and <code>post_stop</code> — prepare env, notify peers, or clean up.</p>
        <a href="/docs/03-orchestration/lifecycle-hooks">Lifecycle hooks →</a>
      </article>
      <article class="home-feature">
        <h3>Event Hooks</h3>
        <p>React to <code>process_fatal</code>, <code>process_started</code>, and other system events with local scripts — Supervisor-style listeners, API-driven.</p>
        <a href="/docs/03-orchestration/event-hooks">Event hooks →</a>
      </article>
      <article class="home-feature">
        <h3>Auto-Recovery</h3>
        <p>Supervisor-compatible <code>autorestart</code>, <code>exitcodes</code>, and <code>startsecs</code> — familiar semantics for migrations.</p>
        <a href="/docs/04-production-scenarios/migrations/vs-supervisor">vs Supervisor →</a>
      </article>
      <article class="home-feature">
        <h3>Cron Scheduling</h3>
        <p>Run programs on cron expressions — periodic jobs without an external scheduler or crontab.</p>
        <a href="/docs/02-essentials/scheduled-tasks">Scheduled tasks →</a>
      </article>
      <article class="home-feature">
        <h3>Remote Ops over HTTP</h3>
        <p>One REST API for CLI, scripts, and remote control of programs, logs, and OTA. Community (OSS) has <strong>no API authentication</strong> — keep the bind on loopback. Licensed deployments add Bearer auth via the security plugin.</p>
        <a href="/docs/04-production-scenarios/observability/programmatic-control">Programmable ops →</a>
      </article>
    </div>
  </section>

  {{< home-premium >}}

  <section class="home-section">
    <h2 class="home-section-title">API-First</h2>
    <div class="home-split">
      <div class="home-split-text">
        <h3>REST &amp; WebSockets</h3>
        <p>Register and control programs over HTTP — the CLI uses the same endpoints. Drive local or remote ops from scripts and CI; stream logs via WebSockets.</p>
        <a href="/docs/06-internals/api-reference">Full API reference →</a>
      </div>
      <div class="home-split-code">
        <pre><code>curl -X POST http://127.0.0.1:9002/api/v1/programs \
  -H "Content-Type: application/json" \
  -d '{
    "name": "api-server",
    "command": "./app",
    "autostart": true,
    "autorestart": "unexpected"
  }'</code></pre>
      </div>
    </div>
  </section>
