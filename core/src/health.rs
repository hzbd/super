use common::HealthCheck;
use std::time::Duration;
use std::sync::OnceLock;
use tokio::net::TcpStream;

// Shared HTTP client; avoid rebuilding connection pool per check
static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn get_http_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .user_agent("Superd-HealthCheck/1.0")
            // Health checks hit localhost/LAN; disable proxy to skip extra hops
            .no_proxy()
            .build()
            .expect("Failed to initialize HealthCheck HTTP Client")
    })
}

/// Run one health check. Returns true if healthy.
pub async fn perform_check(check: &HealthCheck) -> bool {
    match check {
        HealthCheck::Tcp { host, port } => check_tcp(host, *port).await,
        HealthCheck::Http { url, method } => check_http(url, method.as_deref()).await,
        HealthCheck::Exec { command } => check_exec(command).await,

        HealthCheck::Disabled => true,
    }
}

async fn check_tcp(host: &str, port: u16) -> bool {
    let addr = format!("{}:{}", host, port);
    matches!(
        tokio::time::timeout(Duration::from_secs(3), TcpStream::connect(&addr)).await,
        Ok(Ok(_))
    )
}

async fn check_http(url: &str, method: Option<&str>) -> bool {
    let client = get_http_client();

    let method_str = method.unwrap_or("GET");
    let method = match method_str.to_uppercase().as_str() {
        "POST" => reqwest::Method::POST,
        "HEAD" => reqwest::Method::HEAD,
        "PUT"  => reqwest::Method::PUT,
        _ => reqwest::Method::GET,
    };

    match client.request(method, url).send().await {
        Ok(resp) => resp.status().is_success(), // 2xx
        Err(_) => false,
    }
}

async fn check_exec(command: &str) -> bool {
    // Timeout so a stuck script cannot hang the health checker
    // sh -c supports pipes/redirects (most portable option)
    let check_future = async {
        match tokio::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .kill_on_drop(true) // kill child if future is dropped on timeout
            .status()
            .await
        {
            Ok(status) => status.success(),
            Err(_) => false,
        }
    };

    // 7s timeout
    match tokio::time::timeout(Duration::from_secs(7), check_future).await {
        Ok(result) => result,
        Err(_) => {
            tracing::warn!("Health check exec timed out: '{}'", command);
            false
        }
    }
}
