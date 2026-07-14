use axum::{
    Router,
    http::{HeaderValue, StatusCode, Uri, header},
    response::{IntoResponse, Response},
};
use common::license::{LicenseInfo, superd_within_license};
use super_core::{
    ManagerHandle, api, bootstrap,
    plugin::{PluginHost, attach_http_plugins, load_ui_plugin, normalize_ui_path, validate_licensed_auth_secret, validate_licensed_security, RunMode},
    resolve_root,
};
use tokio::signal;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const OSS_UI_MESSAGE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Super Process Manager</title>
  <style>
    :root { color-scheme: light dark; }
    body {
      font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
      max-width: 42rem;
      margin: 0 auto;
      padding: 3rem 1.25rem 4rem;
      line-height: 1.6;
      background: #fafafa;
    }
    .card {
      background: #fff;
      color: #18181b;
      border: 1px solid #e4e4e7;
      border-radius: 1rem;
      padding: 1.75rem 1.5rem;
      box-shadow: 0 1px 2px rgba(0,0,0,.04);
    }
    h1 { font-size: 1.5rem; margin: 0 0 .5rem; letter-spacing: -0.02em; color: inherit; }
    .lead { font-size: 1.05rem; margin: 0 0 1rem; color: #52525b; }
    p { margin: 0 0 .85rem; color: inherit; }
    ul { margin: .5rem 0 1rem; padding-left: 1.25rem; color: inherit; }
    li { margin: .25rem 0; }
    code { background: #f4f4f5; color: #18181b; padding: .12rem .35rem; border-radius: .25rem; font-size: .92em; }
    .actions { display: flex; flex-wrap: wrap; gap: .65rem; margin-top: 1.25rem; }
    .actions a {
      display: inline-block;
      padding: .55rem 1rem;
      border-radius: .5rem;
      font-size: .925rem;
      font-weight: 600;
      text-decoration: none;
      transition: opacity .15s;
    }
    .actions a:hover { opacity: .88; }
    .cta-primary { background: #4f46e5; color: #fff; }
    .cta-secondary {
      background: #f4f4f5;
      color: #18181b;
      border: 1px solid #e4e4e7;
    }
    .muted { font-size: .85rem; color: #71717a; margin-top: 1.25rem; }
    @media (prefers-color-scheme: dark) {
      body { background: #09090b; }
      .card { background: #18181b; color: #fafafa; border-color: #27272a; }
      .lead { color: #d4d4d8; }
      code { background: #27272a; color: #fafafa; }
      .muted { color: #a1a1aa; }
      .cta-secondary { background: #27272a; color: #fafafa; border-color: #3f3f46; }
    }
  </style>
</head>
<body>
  <div class="card">
    <h1>Super Process Manager</h1>
    <p class="lead"><code>superd</code> is up — you're on the open-source build.</p>
    <p>Everything you need for automation is already here:</p>
    <ul>
      <li><code>super</code> CLI for day-to-day operations</li>
      <li><code>/api/*</code> for integrations and CI/CD</li>
      <li><code>/metrics</code> for Prometheus monitoring</li>
    </ul>
    <p>
      Prefer a visual control plane? A valid subscription key and authorized plugin
      libraries can unlock a <strong>dashboard</strong>, plus optional
      <strong>API authentication</strong>, <strong>notifications</strong>, and
      <strong>resource limits</strong> — same <code>superd</code> binary.
    </p>
    <div class="actions">
      <a class="cta-primary" href="https://super.docs.sconts.com/" rel="noopener noreferrer">Learn about subscriptions</a>
      <a class="cta-secondary" href="http://super.docs.sconts.com/docs/07-editions/feature-matrix" rel="noopener noreferrer">OSS vs licensed</a>
      <a class="cta-secondary" href="http://super.docs.sconts.com/docs/" rel="noopener noreferrer">Overview</a>
    </div>
    <p class="muted">Version VERSION_PLACEHOLDER · MIT open-source core</p>
  </div>
</body>
</html>
"#;

async fn ui_fallback_handler(
    uri: Uri,
    ui: Option<std::sync::Arc<super_core::plugin::UiPluginHandle>>,
    auth_required: bool,
    is_licensed: bool,
) -> Response {
    let path = uri.path();
    if path.starts_with("/api/") || path == "/metrics" || path.starts_with("/ws") {
        return StatusCode::NOT_FOUND.into_response();
    }

    let Some(ui) = ui else {
        let html = OSS_UI_MESSAGE.replace("VERSION_PLACEHOLDER", VERSION);
        return html_response(&html);
    };

    serve_ui_asset(&ui, &normalize_ui_path(path), auth_required, is_licensed, false)
        .unwrap_or_else(|| spa_fallback(&ui, auth_required, is_licensed))
}

fn spa_fallback(
    ui: &super_core::plugin::UiPluginHandle,
    auth_required: bool,
    is_licensed: bool,
) -> Response {
    serve_ui_asset(ui, "index.html", auth_required, is_licensed, true)
        .unwrap_or(StatusCode::NOT_FOUND.into_response())
}

fn serve_ui_asset(
    ui: &super_core::plugin::UiPluginHandle,
    file_path: &str,
    auth_required: bool,
    is_licensed: bool,
    inject_config: bool,
) -> Option<Response> {
    let asset = ui.resolve(file_path)?;
    let body = if inject_config || file_path == "index.html" {
        inject_ui_config(asset.data, auth_required, is_licensed)
    } else {
        bytes::Bytes::copy_from_slice(asset.data)
    };

    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(asset.mime).unwrap_or_else(|_| {
            HeaderValue::from_static("application/octet-stream")
        }),
    );

    Some((headers, body).into_response())
}

fn inject_ui_config(raw_html: &[u8], auth_required: bool, is_licensed: bool) -> bytes::Bytes {
    let html_str = String::from_utf8_lossy(raw_html);
    let edition = if is_licensed { "licensed" } else { "oss" };
    let config_js = format!(
        "window.__SUPER_CONFIG__ = {{ edition: '{edition}', auth_required: {auth_required}, version: '{VERSION}' }};",
        auth_required = auth_required,
    );
    let mut injected =
        html_str.replace("window.__SUPER_CONFIG__ = defaultConfig;", &config_js);
    if injected == html_str {
        injected = html_str.replace("// __INJECT_CONFIG__", &config_js);
    }
    bytes::Bytes::from(injected.into_bytes())
}

fn html_response(html: &str) -> Response {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/html"));
    (headers, html.to_string()).into_response()
}

async fn shutdown_signal(mut rx: tokio::sync::broadcast::Receiver<()>, manager: ManagerHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = rx.recv() => {
            tracing::info!("Internal shutdown signal received. Web server stopping.");
        },
        _ = ctrl_c => {
            tracing::info!("Received Ctrl+C. Initiating graceful shutdown...");
            if let Err(e) = manager.shutdown().await {
                tracing::error!("Manager shutdown failed: {}", e);
            }
        },
        _ = terminate => {
            tracing::info!("Received SIGTERM. Initiating graceful shutdown...");
            if let Err(e) = manager.shutdown().await {
                tracing::error!("Manager shutdown failed: {}", e);
            }
        },
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let root = resolve_root();

    let plugin_host = PluginHost::discover(&root, VERSION);
    let licensed_plugins = plugin_host.licensed_plugins.clone();
    let loaded_plugins = plugin_host.loaded_plugins.clone();
    let is_licensed = plugin_host.is_licensed();
    let mut plugin_runtime = plugin_host.runtime;
    let auth_expected = plugin_runtime.loaded_ids.iter().any(|id| id == "security");
    let extension = plugin_runtime.take_extension();
    let ui_plugin = load_ui_plugin(&plugin_runtime);

    let core = bootstrap(extension).await?;

    validate_licensed_security(
        plugin_host.mode,
        plugin_host.claims.as_ref(),
        &loaded_plugins,
        &plugin_host.installed_plugins,
        &plugin_host.plugins_dir,
    )?;
    validate_licensed_auth_secret(
        plugin_host.mode,
        &loaded_plugins,
        core.config.auth_secret.as_deref(),
    )?;

    if is_licensed {
        tracing::info!(
            "Licensed plugins active: {:?} (loaded: {:?})",
            licensed_plugins,
            loaded_plugins
        );
    }

    if ui_plugin.is_some() {
        tracing::info!("Licensed UI plugin active");
    }

    let license_info = plugin_host.claims.as_ref().map(|claims| {
        let mut info = LicenseInfo::from(claims);
        info.plugin_versions = plugin_runtime.plugin_versions.clone();
        info.superd_version = Some(VERSION.to_string());
        info.version_in_range = Some(superd_within_license(claims, VERSION));
        info
    });
    if license_info.is_some() {
        tracing::info!("License API enabled at GET /api/system/license");
    } else {
        tracing::warn!("No license in AppState; GET /api/system/license will return 404");
    }

    let base_router = api::make_api_router(
        core.manager_handle.clone(),
        core.log_tx,
        core.shutdown_tx,
        core.config.clone(),
        !auth_expected,
        license_info,
    );

    let (api_router, auth_required) =
        attach_http_plugins(base_router, &plugin_runtime, &core.paths)?;

    if auth_required {
        tracing::info!("Plugin HTTP auth middleware active");
    } else if plugin_host.mode == RunMode::Licensed {
        anyhow::bail!(
            "Licensed deployment requires the security plugin HTTP auth middleware, but it is not active. \
             Ensure security.so exports authenticate and re-check superd logs."
        );
    }

    let auth_flag = auth_required;
    let licensed_flag = is_licensed;
    let ui_handle = ui_plugin.clone();
    let app = Router::new().merge(api_router).fallback(move |uri: Uri| {
        let ui = ui_handle.clone();
        async move {
            ui_fallback_handler(uri, ui, auth_flag, licensed_flag).await
        }
    });

    let addr = format!("{}:{}", core.config.server.host, core.config.server.port);

    if !common::is_loopback_bind_host(&core.config.server.host)
        && !auth_required
        && !core.config.server.allow_insecure_public_bind
    {
        anyhow::bail!(
            "Refusing to bind to {} without authentication. \
             Set server.allow_insecure_public_bind = true to acknowledge the risk, \
             bind to 127.0.0.1, or load the security plugin.",
            core.config.server.host
        );
    }

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    if auth_required {
        tracing::info!(
            "Superd listening on {} (plugins: {:?}, auth enabled)",
            addr,
            loaded_plugins
        );
    } else if is_licensed {
        tracing::info!(
            "Superd listening on {} (plugins: {:?})",
            addr,
            loaded_plugins
        );
    } else {
        tracing::info!("Superd (OSS) listening on {}", addr);
    }

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal(core.shutdown_rx, core.manager_handle))
    .await?;

    Ok(())
}
