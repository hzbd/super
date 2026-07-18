---
title: ""
description: "Qué es Project Super y cómo instalarlo y operarlo."
---


<section class="home-hero">
    <h1>Project Super</h1>
    <p class="hero-lead">Orquestador de procesos ligero para edge y servidores — alternativa moderna y API-first a Supervisor o PM2.</p>
    <p><code>superd</code> es un gestor de procesos en Rust: un solo binario. Define programas en TOML o REST API; reinicio automático, orden de dependencias y health checks. CLI, navegador o HTTP API.</p>
  </section>

  <nav class="home-docnav" aria-label="Documentación">
    <ul class="home-docnav-list">
      <li><a href="/docs/">Resumen</a></li>
      <li><a href="/docs/01-getting-started/">Getting Started</a></li>
      <li><a href="/docs/02-essentials/">Core Essentials</a></li>
      <li><a href="/docs/03-orchestration/">Orchestration</a></li>
      <li><a href="/docs/05-advanced-management/">Advanced Management</a></li>
      <li><a href="/docs/06-internals/api-reference/">API Reference</a></li>
    </ul>
  </nav>

  <section class="home-section">
    <h2 class="home-section-title">Capacidades principales</h2>
    <div class="home-grid home-grid--3">
      <article class="home-feature">
        <h3>Orquestación de dependencias</h3>
        <p>Declara <code>depends_on</code> y orden de arranque — Super espera programas upstream saludables.</p>
        <a href="/docs/03-orchestration/dependencies">Dependencias →</a>
      </article>
      <article class="home-feature">
        <h3>Actualizaciones OTA atómicas</h3>
        <p>Descarga, verifica checksum, sustituye artefactos y reinicia; rollback automático si fallan los health checks.</p>
        <a href="/docs/03-orchestration/ota-updates">Flujo OTA →</a>
      </article>
      <article class="home-feature">
        <h3>Health checks</h3>
        <p>Sondas TCP/HTTP programadas. Si fallan, el proceso queda unhealthy y bloquea dependientes hasta recuperarse.</p>
        <a href="/docs/03-orchestration/health-checks">Health checks →</a>
      </article>
      <article class="home-feature">
        <h3>Hooks de ciclo de vida</h3>
        <p>Scripts en <code>pre_start</code>, <code>post_start</code>, <code>post_stop</code>.</p>
        <a href="/docs/03-orchestration/lifecycle-hooks">Hooks de ciclo de vida →</a>
      </article>
      <article class="home-feature">
        <h3>Hooks de eventos</h3>
        <p>Reacciona a <code>process_fatal</code>, <code>process_started</code>, etc. con scripts locales.</p>
        <a href="/docs/03-orchestration/event-hooks">Hooks de eventos →</a>
      </article>
      <article class="home-feature">
        <h3>Auto-recuperación</h3>
        <p>Compatible con Supervisor: <code>autorestart</code>, <code>exitcodes</code>, <code>startsecs</code>.</p>
        <a href="/docs/04-production-scenarios/migrations/vs-supervisor">vs Supervisor →</a>
      </article>
      <article class="home-feature">
        <h3>Programación Cron</h3>
        <p>Ejecuta programas con expresiones cron sin scheduler externo.</p>
        <a href="/docs/02-essentials/scheduled-tasks">Tareas programadas →</a>
      </article>
      <article class="home-feature">
        <h3>Operaciones remotas por HTTP</h3>
        <p>Una sola API REST para CLI, scripts y control remoto de programas, logs y OTA. Community (OSS) <strong>no tiene autenticación de API</strong> — mantén el bind en loopback. Las ediciones con licencia añaden Bearer con el plugin security.</p>
        <a href="/docs/04-production-scenarios/observability/programmatic-control">Ops programables →</a>
      </article>
    </div>
  </section>

  {{< home-premium >}}

  <section class="home-section">
    <h2 class="home-section-title">API-first</h2>
    <div class="home-split">
      <div class="home-split-text">
        <h3>REST y WebSockets</h3>
        <p>Registra y controla programas por HTTP; la CLI usa los mismos endpoints. Scripts y CI para ops locales o remotas; logs por WebSocket.</p>
        <a href="/docs/06-internals/api-reference">Referencia API completa →</a>
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
