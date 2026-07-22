---
title: ""
description: "Что такое Project Super и как его установить и использовать."
---


<section class="home-hero">
    <h1>Project Super</h1>
    <p class="hero-lead">Лёгкий оркестратор процессов для edge и серверов — современная API-first альтернатива Supervisor и PM2.</p>
    <p>Менеджер процессов на Rust — один бинарник <code>superd</code>. Программы в TOML или REST API: автоперезапуск, порядок зависимостей, health checks. CLI, браузер или HTTP API.</p>
  </section>

  <nav class="home-docnav" aria-label="Документация">
    <ul class="home-docnav-list">
      <li><a href="/docs/">Обзор</a></li>
      <li><a href="/docs/01-getting-started/">Getting Started</a></li>
      <li><a href="/docs/02-essentials/">Core Essentials</a></li>
      <li><a href="/docs/03-orchestration/">Orchestration</a></li>
      <li><a href="/docs/05-advanced-management/">Advanced Management</a></li>
      <li><a href="/docs/06-internals/api-reference/">API Reference</a></li>
    </ul>
  </nav>

  <section class="home-section">
    <h2 class="home-section-title">Ключевые возможности</h2>
    <div class="home-grid home-grid--3">
      <article class="home-feature">
        <h3>Оркестрация зависимостей</h3>
        <p>Объявите <code>depends_on</code> и порядок запуска — Super ждёт готовности upstream.</p>
        <a href="/docs/03-orchestration/dependencies">Зависимости →</a>
      </article>
      <article class="home-feature">
        <h3>Атомарные OTA-обновления</h3>
        <p>Загрузка, проверка checksum, замена артефактов и перезапуск; откат при сбое health checks.</p>
        <a href="/docs/03-orchestration/ota-updates">OTA →</a>
      </article>
      <article class="home-feature">
        <h3>Health checks</h3>
        <p>TCP/HTTP-пробы по расписанию; при сбое процесс unhealthy, зависимости ждут успешной проверки.</p>
        <a href="/docs/03-orchestration/health-checks">Health checks →</a>
      </article>
      <article class="home-feature">
        <h3>Хуки жизненного цикла</h3>
        <p>Скрипты на <code>pre_start</code>, <code>post_start</code>, <code>post_stop</code>.</p>
        <a href="/docs/03-orchestration/lifecycle-hooks">Хуки жизненного цикла →</a>
      </article>
      <article class="home-feature">
        <h3>Хуки событий</h3>
        <p>Реакция на <code>process_fatal</code>, <code>process_started</code> и др. локальными скриптами.</p>
        <a href="/docs/03-orchestration/event-hooks">Хуки событий →</a>
      </article>
      <article class="home-feature">
        <h3>Автовосстановление</h3>
        <p>Совместимость с Supervisor: <code>autorestart</code>, <code>exitcodes</code>, <code>startsecs</code>.</p>
        <a href="/docs/04-production-scenarios/migrations/vs-supervisor">vs Supervisor →</a>
      </article>
      <article class="home-feature">
        <h3>Cron-планирование</h3>
        <p>Запуск по cron-выражениям без внешнего планировщика.</p>
        <a href="/docs/02-essentials/scheduled-tasks">Задачи по расписанию →</a>
      </article>
      <article class="home-feature">
        <h3>Удалённое управление по HTTP</h3>
        <p>Один REST API для CLI, скриптов и удалённого контроля программ, логов и OTA. Community (OSS) — <strong>без аутентификации API</strong>; держите bind на loopback. В лицензии Bearer даёт плагин security.</p>
        <a href="/docs/04-production-scenarios/observability/programmatic-control">Программируемые операции →</a>
      </article>
    </div>
  </section>

  {{< home-premium >}}

  <section class="home-section">
    <h2 class="home-section-title">API-first</h2>
    <div class="home-split">
      <div class="home-split-text">
        <h3>REST и WebSocket</h3>
        <p>Регистрация и управление по HTTP; CLI использует те же endpoints. Скрипты и CI — локально или удалённо; логи через WebSocket.</p>
        <a href="/docs/06-internals/api-reference">Полная справка API →</a>
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
