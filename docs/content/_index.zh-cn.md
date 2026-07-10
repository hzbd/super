---
title: ""
description: "Project Super 是什么，以及如何安装与使用。"
---
<link rel="stylesheet" href="/css/docs-home.css">

<div class="docs-home">

  <section class="home-hero">
    <h1>Project Super</h1>
    <p class="hero-lead">面向边缘设备与服务器的轻量级进程编排器 —— 现代化的 API 优先替代方案，可取代 Supervisor 或 PM2。</p>
    <p><code>superd</code> 是用 Rust 写的进程管理器，只需部署一个二进制。在 TOML 或 REST API 里定义要跑的程序，负责自动重启、按依赖顺序启动、健康检查。CLI、浏览器、HTTP API 均可操作。</p>
  </section>

  <nav class="home-nav">
    <a href="/docs/">文档中心</a>
    <a href="/docs/01-getting-started/">快速开始</a>
    <a href="/docs/02-essentials/">基础</a>
    <a href="/docs/03-orchestration/">编排</a>
    <a href="/docs/05-advanced-management/">高级管理</a>
    <a href="/docs/06-internals/api-reference/">API 参考</a>
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
    </div>
  </section>

  <section class="home-section home-section--premium">
    <h2 class="home-section-title">
      授权插件
      <span class="home-section-premium-tag"><span aria-hidden="true">💎</span> 商业版</span>
    </h2>
    <p class="home-section-lead">同一 <code>superd</code> 二进制 — 放置 <code>plugins/*.so</code> 并在 <code>conf/super.toml</code> 中配置有效的 <code>[license].key</code> 即可启用。</p>
    <p><strong>预发布状态：</strong>插件 + 订阅模型仍在开发中，<strong>暂不对客户交付</strong>。下文文档描述的是目标架构。</p>
    <div class="home-grid home-grid--3">
      <article class="home-feature home-feature--premium">
        <span class="premium-badge"><span class="premium-badge-icon" aria-hidden="true">💎</span> 插件</span>
        <h3>API 认证</h3>
        <p>每个端点 Bearer 令牌认证；控制台与 API 需登录，可安全暴露到 localhost 之外。</p>
        <a href="/docs/05-advanced-management/authentication">认证 →</a>
      </article>
      <article class="home-feature home-feature--premium">
        <span class="premium-badge"><span class="premium-badge-icon" aria-hidden="true">💎</span> 插件</span>
        <h3>RBAC</h3>
        <p>面向运维与租户的角色访问控制，限制谁可以启停或改配置。</p>
        <a href="/docs/05-advanced-management/access-control">访问控制 →</a>
      </article>
      <article class="home-feature home-feature--premium">
        <span class="premium-badge"><span class="premium-badge-icon" aria-hidden="true">💎</span> 插件</span>
        <h3>Cgroup 隔离</h3>
        <p>Linux cgroups v2 按程序限制 CPU/内存，抑制共享主机上的 noisy neighbor。</p>
        <a href="/docs/05-advanced-management/resource-isolation">资源隔离 →</a>
      </article>
      <article class="home-feature home-feature--premium">
        <span class="premium-badge"><span class="premium-badge-icon" aria-hidden="true">💎</span> 插件</span>
        <h3>Webhook 通知</h3>
        <p>在 <code>process_fatal</code> 等事件推送到 Slack、钉钉、飞书或自定义 URL。</p>
        <a href="/docs/05-advanced-management/event-notifications">事件通知 →</a>
      </article>
      <article class="home-feature home-feature--premium">
        <span class="premium-badge"><span class="premium-badge-icon" aria-hidden="true">💎</span> 插件</span>
        <h3>审计日志</h3>
        <p>不可篡改的操作记录 —— API 调用、配置变更与运维操作，满足合规需求。</p>
        <a href="/docs/05-advanced-management/operation-audit">操作审计 →</a>
      </article>
    </div>
  </section>

  <section class="home-section">
    <h2 class="home-section-title">内置控制台</h2>
    <div class="home-grid home-grid--2">
      <article class="home-card">
        <div class="home-card-media">
          <img src="/images/oss_dash.01.png" alt="进程列表与主机指标">
        </div>
        <div class="home-card-body">
          <h3>概览</h3>
          <p>进程状态、主机 CPU/内存曲线与启停控制 —— 由 <code>superd</code> 直接提供，无需额外 Web 服务器。</p>
        </div>
      </article>
      <article class="home-card">
        <div class="home-card-media">
          <img src="/images/oss_dash.02.png" alt="程序详情与实时日志">
        </div>
        <div class="home-card-body">
          <h3>日志与详情</h3>
          <p>实时 stdout/stderr、钩子、健康检查与环境变量，集中在一个抽屉面板。</p>
        </div>
      </article>
    </div>
  </section>

  <section class="home-section">
    <h2 class="home-section-title">API 优先</h2>
    <div class="home-split">
      <div class="home-split-text">
        <h3>REST 与 WebSocket</h3>
        <p>通过 HTTP 注册与控制程序 —— CLI 使用相同端点；日志经 WebSocket 流式传输。</p>
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

</div>
