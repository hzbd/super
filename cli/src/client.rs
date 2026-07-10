use common::{ProcessStatus, ProgramInfo, ProgramSummary, WsMessage};
use futures_util::StreamExt;
use reqwest::header;
use std::io::Write;
use std::time::{Duration, Instant};
use tokio_tungstenite::connect_async;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub enum WaitTarget {
    Up,   // Running/Healthy
    Down, // Stopped

    // Restart-only: carries the previous PID
    // None if stopped before restart; Some holds the previous PID
    Restarted(Option<u32>),
}

/// Build an HTTP client with optional Bearer token.
pub fn build_client(token: Option<&String>) -> anyhow::Result<reqwest::Client> {
    let mut headers = header::HeaderMap::new();
    if let Some(t) = token {
        let mut auth_val = header::HeaderValue::from_str(&format!("Bearer {t}"))?;
        auth_val.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_val);
    }
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    Ok(client)
}

/// Verify credentials against a server with the security plugin (GET /api/auth/tokens).
pub async fn verify_credentials(base_url: &str, token: &str) -> anyhow::Result<()> {
    let client = build_client(Some(&token.to_string()))?;
    let url = format!("{}/api/auth/tokens", base_url.trim_end_matches('/'));
    let resp = client.get(&url).send().await?;

    match resp.status() {
        reqwest::StatusCode::OK => Ok(()),
        reqwest::StatusCode::UNAUTHORIZED => Err(anyhow::anyhow!("Login failed: invalid token.")),
        reqwest::StatusCode::NOT_FOUND => Err(anyhow::anyhow!(
            "Login requires superd with the security plugin loaded. \
             This server ({base_url}) returned 404 for /api/auth/tokens."
        )),
        status => Err(anyhow::anyhow!("Login failed: server returned {status}.")),
    }
}

/// Resolve target (all, @group, name, id)
pub async fn resolve_targets(
    client: &reqwest::Client,
    base_url: &str,
    target: &str,
) -> anyhow::Result<Vec<Uuid>> {
    let url = format!("{}/api/programs", base_url);
    let resp = client.get(&url).send().await?;

    if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
        return Err(anyhow::anyhow!(
            "Error: Unauthorized. Run `super login`, or pass --token / set SUPER_TOKEN."
        ));
    }
    let programs: Vec<ProgramSummary> = resp.json().await?;

    if target == "all" {
        if programs.is_empty() {
            return Err(anyhow::anyhow!("No programs found on server."));
        }
        return Ok(programs.into_iter().map(|p| p.id).collect());
    }

    if let Some(group_name) = target.strip_prefix('@') {
        let matched: Vec<Uuid> = programs
            .iter()
            .filter(|p| p.group.as_deref() == Some(group_name))
            .map(|p| p.id)
            .collect();
        if matched.is_empty() {
            return Err(anyhow::anyhow!(
                "Error: No programs found in group '@{}'",
                group_name
            ));
        }
        return Ok(matched);
    }

    let matches: Vec<_> = programs
        .iter()
        .filter(|p| p.name == target || p.id.to_string().starts_with(target))
        .collect();

    match matches.len() {
        0 => Err(anyhow::anyhow!("Error: Program not found: '{}'", target)),
        1 => Ok(vec![matches[0].id]),
        _ => {
            eprintln!(
                "Error: Ambiguous target '{}'. Found multiple matches:",
                target
            );
            for p in matches {
                eprintln!("   {} ({})", p.id, p.name);
            }
            Err(anyhow::anyhow!("Please be more specific."))
        }
    }
}

/// Poll until target status is reached
pub async fn wait_for_status(
    client: &reqwest::Client,
    base_url: &str,
    id: Uuid,
    target: WaitTarget,
    timeout_sec: u64,
) -> anyhow::Result<()> {
    let start_time = Instant::now();
    let timeout = Duration::from_secs(timeout_sec);
    let url = format!("{}/api/programs/{}", base_url, id);

    print!("   Verifying status...");
    let _ = std::io::stdout().flush();

    loop {
        if start_time.elapsed() > timeout {
            println!();
            return Err(anyhow::anyhow!(
                "Timeout: Status did not change within {}s.",
                timeout_sec
            ));
        }

        let resp = client.get(&url).send().await?;
        if !resp.status().is_success() {
            println!();
            return Err(anyhow::anyhow!("API Error during verification."));
        }
        let info: ProgramInfo = resp.json().await?;
        let current_state = info.state;
        let current_pid = info.pid;

        match target {
            WaitTarget::Up => {
                match current_state {
                    ProcessStatus::Running | ProcessStatus::Healthy => {
                        println!(" Confirmed (Running, PID: {:?}).", current_pid.unwrap_or(0));
                        return Ok(());
                    }
                    ProcessStatus::Fatal => {
                        println!(" Failed (Crashed/Fatal).");
                        return Err(anyhow::anyhow!("Process crashed immediately."));
                    }
                    ProcessStatus::Backoff => {
                        println!(" Unstable (Backoff).");
                        return Err(anyhow::anyhow!("Process is restarting (Backoff)."));
                    }
                    _ => {} // Waiting, Starting, etc.
                }
            }
            WaitTarget::Down => {
                if current_state == ProcessStatus::Stopped {
                    println!(" Confirmed (Stopped).");
                    return Ok(());
                }
            }
            WaitTarget::Restarted(old_pid) => {
                if matches!(current_state, ProcessStatus::Fatal | ProcessStatus::Backoff) {
                    println!(" Failed (Crashed during restart).");
                    return Err(anyhow::anyhow!("Process crashed during restart."));
                }

                if matches!(
                    current_state,
                    ProcessStatus::Running | ProcessStatus::Healthy
                ) {
                    match (old_pid, current_pid) {
                        (Some(old), Some(new)) if old != new => {
                            println!(" Confirmed (Restarted, PID: {} -> {}).", old, new);
                            return Ok(());
                        }
                        (None, Some(new)) => {
                            println!(" Confirmed (Started, PID: {}).", new);
                            return Ok(());
                        }
                        _ => {}
                    }
                }
            }
        }

        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

/// Stream logs over WebSocket
pub async fn monitor_logs(base_url: &str, id: Uuid, token_query: &str) -> anyhow::Result<()> {
    let ws_base = base_url
        .replace("http://", "ws://")
        .replace("https://", "wss://");
    let ws_url = format!("{}/ws?id={}{}", ws_base, id, token_query);

    println!("Connecting to logs for {}...", id);
    let url = Url::parse(&ws_url)?;
    let (mut ws_stream, resp) = connect_async(url).await?;

    println!("Connected (Status: {})", resp.status());
    println!("---------------------------------------------------");

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() {
            let text = msg.to_text()?;
            if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(text) {
                match ws_msg {
                    WsMessage::Log { source, line, .. } => {
                        let prefix = if source == "stderr" { "[ERR]" } else { "[OUT]" };
                        println!("{} {}", prefix, line);
                    }
                    WsMessage::StatusChange { status, .. } => {
                        println!("[SYS] Status changed to: {:?}", status);
                    }
                }
            }
        } else if msg.is_close() {
            println!("Server closed connection.");
            break;
        }
    }
    Ok(())
}
