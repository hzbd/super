---
title: ""
description: "Project Super 是什么，以及如何安装与使用。"
---


<section class="home-hero">
    <h1>Project Super</h1>
    <p class="hero-lead">面向边缘设备与服务器的轻量级进程编排器 —— 现代化的 API 优先替代方案，可取代 Supervisor 或 PM2。</p>
    <p><code>superd</code> 是用 Rust 写的进程管理器，只需部署一个二进制。在 TOML 或 REST API 里定义要跑的程序，负责自动重启、按依赖顺序启动、健康检查。CLI、浏览器、HTTP API 均可操作。</p>
  </section>

  <nav class="home-docnav" aria-label="文档导航">
    <ul class="home-docnav-list">
      <li><a href="/docs/">概览</a></li>
      <li><a href="/docs/01-getting-started/">快速开始</a></li>
      <li><a href="/docs/02-essentials/">基础</a></li>
      <li><a href="/docs/03-orchestration/">编排</a></li>
      <li><a href="/docs/05-advanced-management/">高级管理</a></li>
      <li><a href="/docs/06-internals/api-reference/">API 参考</a></li>
    </ul>
  </nav>

  <section class="home-section">
    <h2 class="home-section-title">核心能力</h2>
    <div class="home-grid home-grid--3">
      <article class="home-feature">
        <h3>依赖编排</h3>
        <p>声明 <code>depends_on</code> 与启动顺序 —— Super 等待上游程序健康后再启动依赖项。</p>
        <a href="/docs/03-orchestration/dependencies">依赖关系 →</a>
      </article>
      <article class="home-feature">
        <h3>原子 OTA 更新</h3>
        <p>下载、校验、替换制品并重启；健康检查失败时自动回滚。</p>
        <a href="/docs/03-orchestration/ota-updates">OTA 流程 →</a>
      </article>
      <article class="home-feature">
        <h3>健康检查</h3>
        <p>按计划执行 TCP/HTTP 探测；失败时标记为不健康，依赖项会等待直至检查恢复。</p>
        <a href="/docs/03-orchestration/health-checks">健康检查 →</a>
      </article>
      <article class="home-feature">
        <h3>生命周期钩子</h3>
        <p>在 <code>pre_start</code>、<code>post_start</code>、<code>post_stop</code> 运行脚本 —— 准备环境、通知或对端、清理资源。</p>
        <a href="/docs/03-orchestration/lifecycle-hooks">生命周期钩子 →</a>
      </article>
      <article class="home-feature">
        <h3>事件钩子</h3>
        <p>对 <code>process_fatal</code>、<code>process_started</code> 等系统事件执行本地脚本 —— 类 Supervisor 监听，由 API 驱动。</p>
        <a href="/docs/03-orchestration/event-hooks">事件钩子 →</a>
      </article>
      <article class="home-feature">
        <h3>自动恢复</h3>
        <p>兼容 Supervisor 的 <code>autorestart</code>、<code>exitcodes</code>、<code>startsecs</code> —— 迁移语义熟悉。</p>
        <a href="/docs/04-production-scenarios/migrations/vs-supervisor">对比 Supervisor →</a>
      </article>
      <article class="home-feature">
        <h3>Cron 调度</h3>
        <p>按 cron 表达式运行程序 —— 无需外部调度器或 crontab。</p>
        <a href="/docs/02-essentials/scheduled-tasks">定时任务 →</a>
      </article>
      <article class="home-feature">
        <h3>HTTP 远程运维</h3>
        <p>同一套 REST API 覆盖 CLI、脚本与远程管控（进程、日志、OTA）。社区版（OSS）<strong>无 API 认证</strong>，请保持本机回环绑定。订阅版通过 security 插件提供 Bearer 鉴权。</p>
        <a href="/docs/04-production-scenarios/observability/programmatic-control">可编程运维 →</a>
      </article>
    </div>
  </section>

  {{< home-premium >}}

  <section class="home-section">
    <h2 class="home-section-title">API 优先</h2>
    <div class="home-split">
      <div class="home-split-text">
        <h3>REST 与 WebSocket</h3>
        <p>通过 HTTP 注册与控制程序 —— CLI 使用相同端点；脚本与 CI 可做本地或远程运维；日志经 WebSocket 流式传输。</p>
        <a href="/docs/06-internals/api-reference">完整 API 参考 →</a>
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
