use common::ArtifactConfig;
use common::security::{FetchUrlPolicy, validate_outbound_url};
use futures_util::StreamExt;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Download phase (enhanced):
/// 1. Automatic retry with exponential backoff
/// 2. Fine-grained timeouts (connect vs transfer)
/// 3. Smart error handling (no retry on fatal 4xx)
pub async fn download_to_staging(
    config: &ArtifactConfig,
    timeout_secs: u64,
) -> anyhow::Result<PathBuf> {
    validate_outbound_url(&config.source, FetchUrlPolicy::OtaArtifact)?;

    let target_path = PathBuf::from(&config.destination);

    // Ensure parent directory exists
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent).await?;
    }

    // Naming: app -> app.new (same dir for atomic rename)
    let file_name = target_path
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("Invalid destination path"))?
        .to_string_lossy();
    let staging_path = target_path.with_file_name(format!("{}.new", file_name));

    tracing::info!("Downloading OTA update to {:?}", staging_path);

    // 1. Build dedicated client
    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(10)) // fixed 10s connect timeout
        .timeout(Duration::from_secs(timeout_secs)) // configurable transfer timeout
        .build()?;

    let max_retries = 3;
    let mut attempt = 0;

    // 2. Retry loop
    loop {
        match perform_download(&client, &config.source, &staging_path).await {
            Ok(_) => break, // download succeeded
            Err(e) => {
                attempt += 1;

                // Decide whether retry is worthwhile
                let is_fatal = if let Some(status) = e
                    .downcast_ref::<reqwest::Error>()
                    .and_then(|re| re.status())
                {
                    // 4xx (e.g. 404, 403) is usually a non-recoverable config error
                    status.is_client_error()
                } else {
                    false
                };

                if is_fatal || attempt > max_retries {
                    tracing::error!(
                        "Download failed permanently after {} attempts: {}",
                        attempt,
                        e
                    );
                    // Remove any leftover empty staging file
                    let _ = fs::remove_file(&staging_path).await;
                    return Err(e);
                }

                // Exponential backoff: 1s -> 2s -> 4s
                let wait_secs = 2u64.pow(attempt as u32 - 1);
                tracing::warn!(
                    "Download failed: {}. Retrying in {}s (Attempt {}/{})",
                    e,
                    wait_secs,
                    attempt,
                    max_retries
                );
                tokio::time::sleep(Duration::from_secs(wait_secs)).await;
            }
        }
    }

    // Verification phase
    tracing::info!("Verifying checksum...");
    let mut file = fs::File::open(&staging_path).await?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];
    loop {
        let n = file.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    let calculated_hash = hex::encode(hasher.finalize());

    if calculated_hash != config.checksum {
        let _ = fs::remove_file(&staging_path).await;
        return Err(anyhow::anyhow!(
            "Checksum mismatch! Expected: {}, Got: {}",
            config.checksum,
            calculated_hash
        ));
    }

    // Set execute permission (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&staging_path).await?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&staging_path, perms).await?;
    }

    Ok(staging_path)
}

// Internal helper: single download attempt.
// File::create truncates on each retry to avoid appending stale data.
async fn perform_download(client: &reqwest::Client, url: &str, path: &Path) -> anyhow::Result<()> {
    let response = client.get(url).send().await?;

    // Check status manually; reqwest does not treat 4xx/5xx as Err by default
    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            // Wrap as reqwest error for outer status check (via error_for_status)
            response.error_for_status().unwrap_err()
        ));
    }

    let mut file = fs::File::create(path).await?;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk).await?;
    }
    file.flush().await?;
    Ok(())
}

/// Backup phase: prefer hard link (fast, atomic); fall back to copy.
pub async fn create_backup(target: &Path) -> anyhow::Result<PathBuf> {
    let file_name = target
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("Invalid path"))?;
    let backup = target.with_file_name(format!("{}.bak", file_name.to_string_lossy()));

    if target.exists() {
        if backup.exists() {
            let _ = fs::remove_file(&backup).await;
        }

        if fs::hard_link(target, &backup).await.is_err() {
            tracing::warn!("Hardlink failed, falling back to copy for backup.");
            fs::copy(target, &backup).await?;
        }
    }
    Ok(backup)
}

/// Apply update (atomic overwrite via rename).
pub async fn apply_update(target: &Path, staging: &Path) -> anyhow::Result<()> {
    if !staging.exists() {
        return Err(anyhow::anyhow!("Staging file missing"));
    }
    fs::rename(staging, target).await?;
    Ok(())
}

/// Rollback: restore backup to target path.
pub async fn rollback(target: &Path, backup: &Path) -> anyhow::Result<()> {
    if !backup.exists() {
        return Err(anyhow::anyhow!("Backup file missing, cannot rollback!"));
    }
    tracing::warn!("Rolling back binary from {:?} to {:?}", backup, target);
    fs::rename(backup, target).await?;
    Ok(())
}

/// Commit transaction: delete backup file.
pub async fn commit(backup: &Path) {
    if backup.exists() {
        let _ = fs::remove_file(backup).await;
    }
}
