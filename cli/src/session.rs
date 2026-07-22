use crate::client;
use crate::config::CliConfig;

pub async fn login(config: &mut CliConfig, secret: &str, url: Option<&str>) -> anyhow::Result<()> {
    let target_url = url
        .unwrap_or(&config.server_url)
        .trim_end_matches('/')
        .to_string();

    client::verify_credentials(&target_url, secret).await?;

    config.server_url = target_url;
    config.auth_token = Some(secret.to_string());
    config.save()?;

    println!("Login successful. Credentials saved to ~/.super/cli.json");
    Ok(())
}

pub async fn logout(config: &mut CliConfig) -> anyhow::Result<()> {
    if config.auth_token.is_none() {
        println!("Not logged in.");
        return Ok(());
    }

    if let Some(token) = config.auth_token.clone() {
        let base = config.server_url.trim_end_matches('/');
        // Best-effort: end sticky root session when logging out with auth_secret.
        let _ = client::server_logout(base, &token).await;
    }

    config.auth_token = None;
    config.save()?;
    println!("Logged out.");
    Ok(())
}
