---
title: ""
description: "Project Super とは何か、インストールと運用方法。"
---
<link rel="stylesheet" href="/css/docs-home.css">

<div class="docs-home">

  <section class="home-hero">
    <h1>Project Super</h1>
    <p class="hero-lead">エッジデバイスとサーバー向けの軽量プロセスオーケストレーター — Supervisor や PM2 に代わる API ファーストの現代的な選択肢。</p>
    <p>Rust 製プロセスマネージャ <code>superd</code> — デプロイは 1 バイナリ。TOML または REST API でプログラムを定義。自動再起動、依存順起動、ヘルスチェック。CLI、ブラウザ、HTTP API で操作。</p>
  </section>

  <nav class="home-nav">
    <a href="/docs/">ドキュメント</a>
    <a href="/docs/01-getting-started/">Getting Started</a>
    <a href="/docs/02-essentials/">Core Essentials</a>
    <a href="/docs/03-orchestration/">Orchestration</a>
    <a href="/docs/05-advanced-management/">Advanced Management</a>
    <a href="/docs/06-internals/api-reference/">API Reference</a>
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
    </div>
  </section>

  <section class="home-section home-section--premium">
    <h2 class="home-section-title">
      ライセンスプラグイン
      <span class="home-section-premium-tag"><span aria-hidden="true">💎</span> 商用</span>
    </h2>
    <p class="home-section-lead">同一の <code>superd</code> バイナリ — <code>plugins/*.so</code> と <code>conf/super.toml</code> の有効な <code>[license].key</code> で有効化。</p>
    <div class="home-grid home-grid--3">
      <article class="home-feature home-feature--premium">
        <span class="premium-badge"><span class="premium-badge-icon" aria-hidden="true">💎</span> プラグイン</span>
        <h3>API 認証</h3>
        <p>全エンドポイントで Bearer トークン。ダッシュボードと API はログイン必須。</p>
        <a href="/docs/05-advanced-management/authentication">認証 →</a>
      </article>
      <article class="home-feature home-feature--premium">
        <span class="premium-badge"><span class="premium-badge-icon" aria-hidden="true">💎</span> プラグイン</span>
        <h3>RBAC</h3>
        <p>オペレーターとテナント向けロールベースアクセス制御。</p>
        <a href="/docs/05-advanced-management/access-control">アクセス制御 →</a>
      </article>
      <article class="home-feature home-feature--premium">
        <span class="premium-badge"><span class="premium-badge-icon" aria-hidden="true">💎</span> プラグイン</span>
        <h3>Cgroup 分離</h3>
        <p>Linux cgroups v2 による CPU/メモリ上限。</p>
        <a href="/docs/05-advanced-management/resource-isolation">リソース分離 →</a>
      </article>
      <article class="home-feature home-feature--premium">
        <span class="premium-badge"><span class="premium-badge-icon" aria-hidden="true">💎</span> プラグイン</span>
        <h3>Webhook 通知</h3>
        <p>Slack、钉钉、飞书などへ <code>process_fatal</code> 等をプッシュ。</p>
        <a href="/docs/05-advanced-management/event-notifications">イベント通知 →</a>
      </article>
      <article class="home-feature home-feature--premium">
        <span class="premium-badge"><span class="premium-badge-icon" aria-hidden="true">💎</span> プラグイン</span>
        <h3>監査ログ</h3>
        <p>誰が何を変更したかの不変記録 — コンプライアンス向け。</p>
        <a href="/docs/05-advanced-management/operation-audit">操作監査 →</a>
      </article>
    </div>
  </section>

  <section class="home-section">
    <h2 class="home-section-title">組み込みダッシュボード</h2>
    <div class="home-grid home-grid--2">
      <article class="home-card">
        <div class="home-card-media">
          <img src="/images/oss_dash.01.png" alt="プロセス一覧とホストメトリクス">
        </div>
        <div class="home-card-body">
          <h3>概要</h3>
          <p>プロセス状態、CPU/RAM、起動/停止 — <code>superd</code> が提供。</p>
        </div>
      </article>
      <article class="home-card">
        <div class="home-card-media">
          <img src="/images/oss_dash.02.png" alt="プログラム詳細とライブログ">
        </div>
        <div class="home-card-body">
          <h3>ログと詳細</h3>
          <p>stdout/stderr、フック、ヘルスチェックを一画面で。</p>
        </div>
      </article>
    </div>
  </section>

  <section class="home-section">
    <h2 class="home-section-title">API ファースト</h2>
    <div class="home-split">
      <div class="home-split-text">
        <h3>REST と WebSocket</h3>
        <p>HTTP でプログラムを登録・制御。CLI も同じエンドポイント。</p>
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

</div>
