use axum::{
    Router,
    http::{HeaderValue, StatusCode, Uri, header},
    response::IntoResponse,
};
use rust_embed::RustEmbed;
use super_core::{
    ManagerHandle, api, bootstrap,
    plugin::{PluginHost, attach_http_plugins},
    resolve_root,
};
use tokio::signal;

#[derive(RustEmbed)]
#[folder = "../dashboard/dist"]
struct OssAssets;

const VERSION: &str = env!("CARGO_PKG_VERSION");

async fn static_handler_with_auth(uri: Uri, auth_required: bool) -> impl IntoResponse {
    static_handler(uri, auth_required).await
}

async fn static_handler(uri: Uri, auth_required: bool) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/').to_string();

    if path.starts_with("api/") {
        return StatusCode::NOT_FOUND.into_response();
    }

    let file_path = if path.is_empty() { "index.html" } else { &path };

    let get_index_html = || -> Option<String> {
        OssAssets::get("index.html").and_then(|content| {
            std::str::from_utf8(content.data.as_ref()).ok().map(|html_str| {
                let config_js = format!(
                    "window.__SUPER_CONFIG__ = {{ edition: 'oss', auth_required: {}, version: '{}' }};",
                    auth_required, VERSION
                );
                html_str.replace("// __INJECT_CONFIG__", &config_js)
            })
        })
    };

    match OssAssets::get(file_path) {
        Some(content) => {
            if file_path == "index.html"
                && let Some(injected) = get_index_html()
            {
                let mut headers = axum::http::HeaderMap::new();
                headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/html"));
                return (headers, injected).into_response();
            }

            let mime = mime_guess::from_path(file_path).first_or_octet_stream();
            let content_type = HeaderValue::from_str(mime.as_ref())
                .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream"));

            let mut headers = axum::http::HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, content_type);

            (headers, content.data).into_response()
        }
        None => {
            if let Some(injected) = get_index_html() {
                let mut headers = axum::http::HeaderMap::new();
                headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/html"));
                return (headers, injected).into_response();
            }
            StatusCode::NOT_FOUND.into_response()
        }
    }
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

    let core = bootstrap(extension).await?;

    if is_licensed {
        tracing::info!(
            "Licensed plugins active: {:?} (loaded: {:?})",
            licensed_plugins,
            loaded_plugins
        );
    }

    let base_router = api::make_api_router(
        core.manager_handle.clone(),
        core.log_tx,
        core.shutdown_tx,
        core.config.clone(),
        !auth_expected,
    );

    let (api_router, auth_required) =
        attach_http_plugins(base_router, &plugin_runtime, &core.paths)?;

    if auth_required {
        tracing::info!("Plugin HTTP auth middleware active");
    }

    let auth_flag = auth_required;
    let app = Router::new()
        .merge(api_router)
        .fallback(move |uri: Uri| async move { static_handler_with_auth(uri, auth_flag).await });

    let addr = format!("{}:{}", core.config.server.host, core.config.server.port);
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

    let is_public =
        core.config.server.host != "127.0.0.1" && core.config.server.host != "localhost";
    if is_public && !auth_required {
        tracing::warn!("====================================================================");
        tracing::warn!(
            "SECURITY WARNING: Superd API is bound to {} without authentication",
            core.config.server.host
        );
        tracing::warn!(
            "OSS edition has no API authentication. Anyone on the network can manage processes."
        );
        tracing::warn!("Bind to 127.0.0.1, use a firewall, or load the security plugin.");
        tracing::warn!("====================================================================");
    }

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal(core.shutdown_rx, core.manager_handle))
    .await?;

    Ok(())
}
