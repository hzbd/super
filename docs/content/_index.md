---
title: ""
description: "What Project Super is, and how to install and operate it."
---

<link rel="stylesheet" href="/css/docs-home.css">

<div class="docs-home">

  <section class="home-hero">
    <h1>Project Super</h1>
    <p class="hero-lead">A lightweight process orchestrator for edge devices and servers — a modern, API-first alternative to Supervisor or PM2.</p>
    <p><code>superd</code> is a Rust process manager — one binary to deploy. Define programs in TOML or over REST; it handles restarts, startup order, and health checks. Manage via CLI, browser, or HTTP API.</p>
  </section>

  <nav class="home-nav">
    <a href="/docs/">Documentation</a>
    <a href="/docs/01-getting-started/">Getting Started</a>
    <a href="/docs/02-essentials/">Core Essentials</a>
    <a href="/docs/03-orchestration/">Orchestration</a>
    <a href="/docs/05-advanced-management/">Advanced Management</a>
    <a href="/docs/06-internals/api-reference/">API Reference</a>
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
    </div>
  </section>

  <section class="home-section home-section--premium">
    <h2 class="home-section-title">
      Licensed Plugins
      <span class="home-section-premium-tag"><span aria-hidden="true">💎</span> Commercial</span>
    </h2>
    <p class="home-section-lead">Same <code>superd</code> binary — load optional <code>plugins/*.so</code> with a valid <code>[license].key</code> in <code>conf/super.toml</code>.</p>
    <p><strong>Pre-release:</strong> The plugin + subscription model is under active development and <strong>not offered for customer delivery</strong> yet. Documentation below describes the target architecture.</p>
    <div class="home-grid home-grid--3">
      <article class="home-feature home-feature--premium">
        <span class="premium-badge"><span class="premium-badge-icon" aria-hidden="true">💎</span> Licensed</span>
        <h3>API Authentication</h3>
        <p>Bearer token auth on every endpoint. Dashboard and API require login — safe to expose beyond localhost.</p>
        <a href="/docs/05-advanced-management/authentication">Authentication →</a>
      </article>
      <article class="home-feature home-feature--premium">
        <span class="premium-badge"><span class="premium-badge-icon" aria-hidden="true">💎</span> Licensed</span>
        <h3>RBAC</h3>
        <p>Role-based access control for operators and tenants. Limit who can start, stop, or reconfigure programs.</p>
        <a href="/docs/05-advanced-management/access-control">Access control →</a>
      </article>
      <article class="home-feature home-feature--premium">
        <span class="premium-badge"><span class="premium-badge-icon" aria-hidden="true">💎</span> Licensed</span>
        <h3>Cgroup Isolation</h3>
        <p>Linux cgroups v2 CPU and memory limits per program. Contain noisy neighbors on shared hosts.</p>
        <a href="/docs/05-advanced-management/resource-isolation">Resource isolation →</a>
      </article>
      <article class="home-feature home-feature--premium">
        <span class="premium-badge"><span class="premium-badge-icon" aria-hidden="true">💎</span> Licensed</span>
        <h3>Webhook Notifications</h3>
        <p>Push alerts to Slack, DingTalk, Feishu, and custom URLs on <code>process_fatal</code> and other events.</p>
        <a href="/docs/05-advanced-management/event-notifications">Event notifications →</a>
      </article>
      <article class="home-feature home-feature--premium">
        <span class="premium-badge"><span class="premium-badge-icon" aria-hidden="true">💎</span> Licensed</span>
        <h3>Audit Logging</h3>
        <p>Immutable record of who changed what — API calls, config updates, and operator actions for compliance.</p>
        <a href="/docs/05-advanced-management/operation-audit">Operation audit →</a>
      </article>
    </div>
  </section>

  <section class="home-section">
    <h2 class="home-section-title">Built-in Dashboard</h2>
    <div class="home-grid home-grid--2">
      <article class="home-card">
        <div class="home-card-media">
          <img src="/images/oss_dash.01.png" alt="Process list and host metrics">
        </div>
        <div class="home-card-body">
          <h3>Overview</h3>
          <p>Process status, host CPU/RAM sparklines, and start/stop controls — served by <code>superd</code>, no extra web server.</p>
        </div>
      </article>
      <article class="home-card">
        <div class="home-card-media">
          <img src="/images/oss_dash.02.png" alt="Program detail with live logs">
        </div>
        <div class="home-card-body">
          <h3>Logs &amp; Details</h3>
          <p>Live stdout/stderr, hooks, health checks, and environment in one drawer.</p>
        </div>
      </article>
    </div>
  </section>

  <section class="home-section">
    <h2 class="home-section-title">API-First</h2>
    <div class="home-split">
      <div class="home-split-text">
        <h3>REST &amp; WebSockets</h3>
        <p>Register and control programs over HTTP — the CLI uses the same endpoints. Stream logs via WebSockets.</p>
        <a href="/docs/06-internals/api-reference">Full API reference →</a>
      </div>
      <div class="home-split-code">
        <pre><code>curl -X POST http://127.0.0.1:9002/api/programs \
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

</div>
