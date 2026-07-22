//! `super doctor` — one-shot diagnostics for support and first-run triage.
//!
//! Aggregates: CLI version, config-file validation (shared with `super check`),
//! daemon connectivity/health, and license/plugin status.

use crate::check;
use crate::client;
use colored::Colorize;
use common::{HealthResponse, LicenseInfo};

pub async fn run(base_url: &str, token: Option<&String>) -> anyhow::Result<()> {
    println!("{}", "Super Doctor".bold());
    println!("   CLI version:     {}", env!("CARGO_PKG_VERSION"));

    // 1. Config file validation (reuse `super check`; it prints its own report).
    println!("\n{}", "== Configuration ==".bold());
    match check::run(None) {
        Ok(()) => {}
        Err(e) => {
            println!("   {}", format!("config check reported: {e}").yellow());
        }
    }

    // 2. Daemon connectivity + health.
    println!("\n{}", "== Daemon ==".bold());
    let base_url = base_url.trim_end_matches('/');
    println!("   Server URL:      {}", base_url.cyan());

    let client = match client::build_client(token) {
        Ok(c) => c,
        Err(e) => {
            println!("   {}", format!("cannot build HTTP client: {e}").red());
            return Ok(());
        }
    };

    let health_url = format!("{base_url}/health");
    let resp = match client.get(&health_url).send().await {
        Ok(r) => r,
        Err(e) => {
            println!("   Status:          {}", format!("unreachable ({e})").red());
            println!(
                "   Hint: start the daemon (`superd`) or pass --server / edit ~/.super/cli.json"
            );
            return Ok(());
        }
    };

    let http_status = resp.status();
    match resp.json::<HealthResponse>().await {
        Ok(h) => {
            let status_colored = match h.status.as_str() {
                "healthy" => h.status.green(),
                "degraded" => h.status.yellow(),
                other => other.red(),
            };
            println!("   Status:          {status_colored} (HTTP {http_status})");
            for (k, v) in &h.components {
                println!("     - {k}: {v}");
            }
        }
        Err(e) => {
            println!(
                "   Status:          {}",
                format!("HTTP {http_status}, unreadable health body: {e}").yellow()
            );
        }
    }

    // 3. License / edition (404 in OSS mode is expected, not an error).
    println!("\n{}", "== License ==".bold());
    let license_url = format!("{base_url}/api/v1/system/license");
    match client.get(&license_url).send().await {
        Ok(r) if r.status() == reqwest::StatusCode::NOT_FOUND => {
            println!(
                "   Mode:            {}",
                "OSS (no license configured)".cyan()
            );
        }
        Ok(r) if r.status().is_success() => match r.json::<LicenseInfo>().await {
            Ok(info) => {
                println!("   Mode:            {}", "Licensed".green());
                println!("   Issued to:       {}", info.issued_to);
                println!("   Subscription:    {}", info.subscription_status);
                if let Some(v) = &info.superd_version {
                    println!("   superd version:  {v}");
                }
                if let Some(in_range) = info.version_in_range
                    && !in_range
                {
                    println!(
                        "   {}",
                        format!(
                            "superd version outside licensed range (max {})",
                            info.max_superd_version
                        )
                        .yellow()
                    );
                }
                if !info.plugin_versions.is_empty() {
                    println!("   Plugins:");
                    for (id, ver) in &info.plugin_versions {
                        println!("     - {id}: {ver}");
                    }
                }
            }
            Err(e) => println!("   {}", format!("unreadable license body: {e}").yellow()),
        },
        Ok(r) => {
            println!(
                "   {}",
                format!("license endpoint returned HTTP {}", r.status()).yellow()
            );
        }
        Err(e) => {
            println!(
                "   {}",
                format!("license endpoint unreachable: {e}").yellow()
            );
        }
    }

    Ok(())
}
