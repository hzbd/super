use std::collections::HashMap;
use std::process::Stdio;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

/// Run a hook script.
/// Returns Ok(true) on exit 0, Ok(false) on script failure.
pub async fn run_hook(
    hook_cmd: &str,
    context_envs: &HashMap<String, String>
) -> anyhow::Result<bool> {
    run_hook_with_stdin(hook_cmd, context_envs, None, 0).await
}

/// Run a hook script, optionally piping JSON to stdin.
/// `timeout_secs`: 0 means no timeout.
pub async fn run_hook_with_stdin(
    hook_cmd: &str,
    context_envs: &HashMap<String, String>,
    stdin_json: Option<&str>,
    timeout_secs: u64,
) -> anyhow::Result<bool> {
    if hook_cmd.trim().is_empty() {
        return Ok(true);
    }

    tracing::info!("Executing Hook: '{}'", hook_cmd);

    let mut child = Command::new("sh")
        .arg("-c")
        .arg(hook_cmd)
        .envs(context_envs)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    if let Some(json) = stdin_json {
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(json.as_bytes()).await?;
            stdin.write_all(b"\n").await?;
        }
    } else if let Some(stdin) = child.stdin.take() {
        drop(stdin);
    }

    let status = if timeout_secs > 0 {
        match tokio::time::timeout(Duration::from_secs(timeout_secs), child.wait()).await {
            Ok(res) => res?,
            Err(_) => {
                tracing::warn!("Hook timed out after {}s", timeout_secs);
                return Ok(false);
            }
        }
    } else {
        child.wait().await?
    };

    if status.success() {
        tracing::info!("Hook finished successfully.");
        Ok(true)
    } else {
        tracing::warn!("Hook failed with exit code: {:?}", status.code());
        Ok(false)
    }
}
