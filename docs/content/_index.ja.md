---
title: ""
description: "Project Super とは何か、インストールと運用方法。"
---


<section class="home-hero">
    <h1>Project Super</h1>
    <p class="hero-lead">エッジデバイスとサーバー向けの軽量プロセスオーケストレーター — Supervisor や PM2 に代わる API ファーストの現代的な選択肢。</p>
    <p>Rust 製プロセスマネージャ <code>superd</code> — デプロイは 1 バイナリ。TOML または REST API でプログラムを定義。自動再起動、依存順起動、ヘルスチェック。CLI、ブラウザ、HTTP API で操作。</p>
  </section>

  <nav class="home-docnav" aria-label="ドキュメント">
    <ul class="home-docnav-list">
      <li><a href="/docs/">概要</a></li>
      <li><a href="/docs/01-getting-started/">Getting Started</a></li>
      <li><a href="/docs/02-essentials/">Core Essentials</a></li>
      <li><a href="/docs/03-orchestration/">Orchestration</a></li>
      <li><a href="/docs/05-advanced-management/">Advanced Management</a></li>
      <li><a href="/docs/06-internals/api-reference/">API Reference</a></li>
    </ul>
  </nav>

  <section class="home-section">
    <h2 class="home-section-title">コア機能</h2>
    <div class="home-grid home-grid--3">
      <article class="home-feature">
        <h3>依存関係オーケストレーション</h3>
        <p><code>depends_on</code> と起動順序を宣言 — 上流が健全になるまで依存プログラムの起動を待ちます。</p>
        <a href="/docs/03-orchestration/dependencies">依存関係 →</a>
      </article>
      <article class="home-feature">
        <h3>アトミック OTA 更新</h3>
        <p>ダウンロード、チェックサム検証、成果物の入れ替え、再起動 — ヘルスチェック失敗時は自動ロールバック。</p>
        <a href="/docs/03-orchestration/ota-updates">OTA フロー →</a>
      </article>
      <article class="home-feature">
        <h3>ヘルスチェック</h3>
        <p>スケジュールに沿った TCP/HTTP プローブ。失敗時は unhealthy となり、依存プログラムの起動をブロックします。</p>
        <a href="/docs/03-orchestration/health-checks">ヘルスチェック →</a>
      </article>
      <article class="home-feature">
        <h3>ライフサイクルフック</h3>
        <p><code>pre_start</code>、<code>post_start</code>、<code>post_stop</code> でシェルスクリプトを実行。</p>
        <a href="/docs/03-orchestration/lifecycle-hooks">ライフサイクルフック →</a>
      </article>
      <article class="home-feature">
        <h3>イベントフック</h3>
        <p><code>process_fatal</code> などのシステムイベントにローカルスクリプトで反応 — API 駆動。</p>
        <a href="/docs/03-orchestration/event-hooks">イベントフック →</a>
      </article>
      <article class="home-feature">
        <h3>自動復旧</h3>
        <p>Supervisor 互換の <code>autorestart</code>、<code>exitcodes</code>、<code>startsecs</code>。</p>
        <a href="/docs/04-production-scenarios/migrations/vs-supervisor">Supervisor 比較 →</a>
      </article>
      <article class="home-feature">
        <h3>Cron スケジューリング</h3>
        <p>cron 式でプログラムを実行 — 外部スケジューラ不要。</p>
        <a href="/docs/02-essentials/scheduled-tasks">スケジュールタスク →</a>
      </article>
      <article class="home-feature">
        <h3>HTTP リモート運用</h3>
        <p>CLI・スクリプト・リモート制御が同じ REST API（プログラム、ログ、OTA）。コミュニティ版（OSS）には<strong>API 認証がありません</strong> — バインドはループバックのままにしてください。ライセンス版は security プラグインで Bearer 認証を追加します。</p>
        <a href="/docs/04-production-scenarios/observability/programmatic-control">プログラマブル運用 →</a>
      </article>
    </div>
  </section>

  {{< home-premium >}}

  <section class="home-section">
    <h2 class="home-section-title">API ファースト</h2>
    <div class="home-split">
      <div class="home-split-text">
        <h3>REST と WebSocket</h3>
        <p>HTTP でプログラムを登録・制御。CLI も同じエンドポイント。スクリプトや CI からローカル／リモート運用が可能。ログは WebSocket。</p>
        <a href="/docs/06-internals/api-reference">API リファレンス →</a>
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
