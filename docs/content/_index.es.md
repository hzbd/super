---
title: ""
description: "Qué es Project Super y cómo instalarlo y operarlo."
---
<link rel="stylesheet" href="/css/docs-home.css">

<div class="docs-home">

  <section class="home-hero">
    <h1>Project Super</h1>
    <p class="hero-lead">Orquestador de procesos ligero para edge y servidores — alternativa moderna y API-first a Supervisor o PM2.</p>
    <p><code>superd</code> es un gestor de procesos en Rust: un solo binario. Define programas en TOML o REST API; reinicio automático, orden de dependencias y health checks. CLI, navegador o HTTP API.</p>
  </section>

  <nav class="home-nav">
    <a href="/docs/">Documentación</a>
    <a href="/docs/01-getting-started/">Getting Started</a>
    <a href="/docs/02-essentials/">Core Essentials</a>
    <a href="/docs/03-orchestration/">Orchestration</a>
    <a href="/docs/05-advanced-management/">Advanced Management</a>
    <a href="/docs/06-internals/api-reference/">API Reference</a>
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
    </div>
  </section>

  <section class="home-section home-section--premium">
    <h2 class="home-section-title">
      Plugins con licencia
      <span class="home-section-premium-tag"><span aria-hidden="true">💎</span> Comercial</span>
    </h2>
    <p class="home-section-lead">El mismo binario <code>superd</code> — coloque <code>plugins/*.so</code> y un <code>[license].key</code> válido en <code>conf/super.toml</code>.</p>
    <div class="home-grid home-grid--3">
      <article class="home-feature home-feature--premium">
        <span class="premium-badge"><span class="premium-badge-icon" aria-hidden="true">💎</span> Plugin</span>
        <h3>Autenticación API</h3>
        <p>Bearer token en cada endpoint. Panel y API requieren login.</p>
        <a href="/docs/05-advanced-management/authentication">Autenticación →</a>
      </article>
      <article class="home-feature home-feature--premium">
        <span class="premium-badge"><span class="premium-badge-icon" aria-hidden="true">💎</span> Plugin</span>
        <h3>RBAC</h3>
        <p>Control de acceso basado en roles para operadores y tenants.</p>
        <a href="/docs/05-advanced-management/access-control">Control de acceso →</a>
      </article>
      <article class="home-feature home-feature--premium">
        <span class="premium-badge"><span class="premium-badge-icon" aria-hidden="true">💎</span> Plugin</span>
        <h3>Aislamiento Cgroup</h3>
        <p>Límites CPU/memoria con cgroups v2 en Linux.</p>
        <a href="/docs/05-advanced-management/resource-isolation">Aislamiento de recursos →</a>
      </article>
      <article class="home-feature home-feature--premium">
        <span class="premium-badge"><span class="premium-badge-icon" aria-hidden="true">💎</span> Plugin</span>
        <h3>Notificaciones Webhook</h3>
        <p>Alertas a Slack, DingTalk, Feishu y URLs personalizadas.</p>
        <a href="/docs/05-advanced-management/event-notifications">Notificaciones →</a>
      </article>
      <article class="home-feature home-feature--premium">
        <span class="premium-badge"><span class="premium-badge-icon" aria-hidden="true">💎</span> Plugin</span>
        <h3>Registro de auditoría</h3>
        <p>Registro inmutable de cambios — cumplimiento normativo.</p>
        <a href="/docs/05-advanced-management/operation-audit">Auditoría →</a>
      </article>
    </div>
  </section>

  <section class="home-section">
    <h2 class="home-section-title">Panel integrado</h2>
    <div class="home-grid home-grid--2">
      <article class="home-card">
        <div class="home-card-media">
          <img src="/images/oss_dash.01.png" alt="Lista de procesos y métricas">
        </div>
        <div class="home-card-body">
          <h3>Resumen</h3>
          <p>Estado, CPU/RAM y controles — servido por <code>superd</code>, sin servidor web extra.</p>
        </div>
      </article>
      <article class="home-card">
        <div class="home-card-media">
          <img src="/images/oss_dash.02.png" alt="Detalle del programa y logs en vivo">
        </div>
        <div class="home-card-body">
          <h3>Logs y detalles</h3>
          <p>stdout/stderr en vivo, hooks y health checks en un panel.</p>
        </div>
      </article>
    </div>
  </section>

  <section class="home-section">
    <h2 class="home-section-title">API-first</h2>
    <div class="home-split">
      <div class="home-split-text">
        <h3>REST y WebSockets</h3>
        <p>Registra y controla programas por HTTP; la CLI usa los mismos endpoints.</p>
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

</div>
