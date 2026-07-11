use anyhow::Context;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use utoipa::ToSchema;

pub const PUBLIC_KEY_BYTES: &[u8] = include_bytes!("../keys/public.key");

/// Official commercial plugin IDs allowed in signed licenses.
pub const LICENSED_PLUGIN_IDS: &[&str] = &["security", "isolation", "notify", "ui"];

/// Reject unknown plugin IDs before signing a license.
pub fn validate_licensed_plugins(plugins: &[String]) -> anyhow::Result<()> {
    for id in plugins {
        if !LICENSED_PLUGIN_IDS.contains(&id.as_str()) {
            anyhow::bail!("Unknown plugin ID '{id}'. Allowed: {LICENSED_PLUGIN_IDS:?}");
        }
    }
    Ok(())
}

/// Signed license claims. The entire struct is covered by Ed25519 signature.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, ToSchema)]
pub struct LicenseClaims {
    pub issued_to: String,
    pub issued_at: u64,
    pub major_version: u32,
    /// Licensed minor line at issuance (e.g. `2` for Super 1.2.x).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minor_version: Option<u32>,
    /// Highest allowed semver **minor** within `major_version` (cap = `{major}.{max_super_minor}.*`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_super_minor: Option<u32>,
    /// Legacy signed delta above `minor_version`; prefer `max_super_minor` in new keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minor_ahead: Option<u32>,
    /// Super product version this license was issued for (e.g. `"1.2.0"`). Anchors renewals and upgrades.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issued_super_version: Option<String>,
    /// Authorized plugin IDs (e.g. `security`, `notify`, `isolation`).
    pub plugins: Vec<String>,
    /// Unix timestamp when the license expires. Omitted = no expiration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u64>,
    /// When `false`, superd rejects expired keys at runtime. Omitted/`true` = keep plugins offline.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retain_plugins_after_expiry: Option<bool>,
    /// Stable identifier for support / renewal tracking (optional in legacy licenses).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license_id: Option<String>,
}

/// API / dashboard view derived from verified claims.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, ToSchema)]
pub struct LicenseInfo {
    pub issued_to: String,
    pub issued_at: u64,
    pub major_version: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minor_version: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issued_super_version: Option<String>,
    pub plugins: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license_id: Option<String>,
    /// UI feature codes mapped from authorized plugins.
    pub features: Vec<String>,
    /// Loaded plugin release versions (runtime; not part of signed claims).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub plugin_versions: HashMap<String, String>,
    /// `active`, `expired`, or `perpetual` — expired keys keep existing plugins offline.
    pub subscription_status: String,
    /// Product version when the license was issued (e.g. `1.2.0`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issued_for_version: Option<String>,
    /// Highest superd release permanently allowed after subscription ends (e.g. `1.3.x`).
    pub max_superd_version: String,
    /// Deprecated alias for `max_superd_version` (older dashboards).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub supported_super_version: String,
    /// Running superd version (runtime).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub superd_version: Option<String>,
    /// Whether the running superd is within `max_superd_version` (runtime).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_in_range: Option<bool>,
    /// Renewal / upgrade page.
    pub upgrade_url: String,
    /// Highest allowed semver minor within `major_version` (from signed claims).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_super_minor: Option<u32>,
}

impl From<&LicenseClaims> for LicenseInfo {
    fn from(claims: &LicenseClaims) -> Self {
        let subscription_status = match license_expiry_status(claims) {
            Ok(LicenseExpiryStatus::Perpetual) => "perpetual",
            Ok(LicenseExpiryStatus::Active) => "active",
            Ok(LicenseExpiryStatus::Expired) => "expired",
            Err(_) => "unknown",
        };
        let max_superd_version = license_max_superd_version(claims);
        Self {
            issued_to: claims.issued_to.clone(),
            issued_at: claims.issued_at,
            major_version: claims.major_version,
            minor_version: claims.minor_version,
            issued_super_version: claims.issued_super_version.clone(),
            plugins: claims.plugins.clone(),
            expires_at: claims.expires_at,
            license_id: claims.license_id.clone(),
            features: plugins_to_features(&claims.plugins),
            plugin_versions: HashMap::new(),
            subscription_status: subscription_status.to_string(),
            issued_for_version: license_issued_for_version(claims),
            max_superd_version: max_superd_version.clone(),
            supported_super_version: max_superd_version,
            superd_version: None,
            version_in_range: None,
            upgrade_url: LICENSE_UPGRADE_URL.to_string(),
            max_super_minor: licensed_max_super_minor(claims),
        }
    }
}

/// Product version recorded at issuance (e.g. `1.2.0`).
pub fn license_issued_for_version(claims: &LicenseClaims) -> Option<String> {
    if let Some(ref ver) = claims.issued_super_version {
        return Some(ver.clone());
    }
    claims
        .minor_version
        .map(|minor| format!("{}.{}.0", claims.major_version, minor))
}

/// Highest superd release this key permanently authorizes (derived from signed claims).
pub fn license_max_superd_version(claims: &LicenseClaims) -> String {
    match licensed_max_super_minor(claims) {
        Some(max_minor) => format!("{}.{}.x", claims.major_version, max_minor),
        None => format!("{}.x", claims.major_version),
    }
}

pub fn superd_within_license(claims: &LicenseClaims, superd_version: &str) -> bool {
    check_superd_version(claims, superd_version).is_ok()
}

/// Map plugin IDs to dashboard feature codes.
pub fn plugins_to_features(plugins: &[String]) -> Vec<String> {
    let mut features = Vec::new();
    for plugin in plugins {
        match plugin.as_str() {
            "security" => {
                features.push("rbac".into());
                features.push("audit".into());
            }
            "notify" => features.push("notify".into()),
            "isolation" => features.push("cgroups".into()),
            "ui" => features.push("dashboard".into()),
            other => features.push(other.to_string()),
        }
    }
    features.sort();
    features.dedup();
    features
}

#[derive(Debug, Serialize, Deserialize)]
struct LicenseContainer {
    claims: LicenseClaims,
    signature: Vec<u8>,
}

fn current_unix_secs() -> anyhow::Result<u64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}

fn ensure_not_expired(claims: &LicenseClaims) -> anyhow::Result<()> {
    if matches!(license_expiry_status(claims)?, LicenseExpiryStatus::Expired)
        && let Some(expires_at) = claims.expires_at
    {
        let now = current_unix_secs()?;
        anyhow::bail!("License expired at {expires_at} (current time {now})");
    }
    Ok(())
}

/// Subscription expiry derived from signed claims and local clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LicenseExpiryStatus {
    Active,
    Expired,
    Perpetual,
}

pub fn license_expiry_status(claims: &LicenseClaims) -> anyhow::Result<LicenseExpiryStatus> {
    match claims.expires_at {
        None => Ok(LicenseExpiryStatus::Perpetual),
        Some(expires_at) => {
            let now = current_unix_secs()?;
            if now > expires_at {
                Ok(LicenseExpiryStatus::Expired)
            } else {
                Ok(LicenseExpiryStatus::Active)
            }
        }
    }
}

fn verify_signature(license_str: &str, verifying_key: &VerifyingKey) -> anyhow::Result<LicenseClaims> {
    let json_bytes = BASE64
        .decode(license_str.trim())
        .context("Invalid license format (Base64 decode failed)")?;

    let container: LicenseContainer =
        serde_json::from_slice(&json_bytes).context("Invalid license structure")?;

    let signature = Signature::from_slice(&container.signature)
        .map_err(|_| anyhow::anyhow!("Invalid signature format"))?;

    let claims_bytes = serde_json::to_vec(&container.claims)?;

    verifying_key
        .verify(&claims_bytes, &signature)
        .map_err(|_| {
            anyhow::anyhow!("License signature verification failed (invalid or tampered)")
        })?;

    Ok(container.claims)
}

/// Verify a Base64-encoded signed license string (strict: rejects expired keys).
pub fn verify_license(license_str: &str) -> anyhow::Result<LicenseClaims> {
    let key_array: [u8; 32] = PUBLIC_KEY_BYTES
        .try_into()
        .expect("Embedded public key has incorrect size");
    let verifying_key = VerifyingKey::from_bytes(&key_array)
        .map_err(|_| anyhow::anyhow!("Internal error: invalid embedded public key"))?;
    verify_license_with_key(license_str, &verifying_key)
}

fn verify_license_with_key(
    license_str: &str,
    verifying_key: &VerifyingKey,
) -> anyhow::Result<LicenseClaims> {
    let claims = verify_signature(license_str, verifying_key)?;
    ensure_not_expired(&claims)?;
    Ok(claims)
}

/// Verify signature for superd runtime (honours signed `retain_plugins_after_expiry` when set).
pub fn verify_license_for_superd(license_str: &str) -> anyhow::Result<(LicenseClaims, LicenseExpiryStatus)> {
    let key_array: [u8; 32] = PUBLIC_KEY_BYTES
        .try_into()
        .expect("Embedded public key has incorrect size");
    let verifying_key = VerifyingKey::from_bytes(&key_array)
        .map_err(|_| anyhow::anyhow!("Internal error: invalid embedded public key"))?;
    verify_license_for_superd_with_key(license_str, &verifying_key)
}

fn verify_license_for_superd_with_key(
    license_str: &str,
    verifying_key: &VerifyingKey,
) -> anyhow::Result<(LicenseClaims, LicenseExpiryStatus)> {
    let claims = verify_signature(license_str, verifying_key)?;
    let expiry = license_expiry_status(&claims)?;
    if matches!(expiry, LicenseExpiryStatus::Expired)
        && claims.retain_plugins_after_expiry == Some(false)
    {
        ensure_not_expired(&claims)?;
    }
    Ok((claims, expiry))
}

/// Parse the major version from a semver-like string (e.g. "1.1.9" -> 1).
pub fn parse_major_version(version: &str) -> u32 {
    parse_semver(version).map(|(major, _, _)| major).unwrap_or(0)
}

/// Parse `major.minor.patch` from a semver-like string (missing parts default to 0).
pub fn parse_semver(version: &str) -> Option<(u32, u32, u32)> {
    let mut parts = version.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    Some((major, minor, patch))
}

/// Subscription / upgrade page shown when the installed superd exceeds the license.
pub const LICENSE_UPGRADE_URL: &str = "https://super.project.sconts.com";

/// Minor line used for version cap checks (explicit `minor_version`, else parsed from `issued_super_version`).
pub fn licensed_minor_line(claims: &LicenseClaims) -> Option<u32> {
    if let Some(minor) = claims.minor_version {
        return Some(minor);
    }
    claims
        .issued_super_version
        .as_deref()
        .and_then(parse_semver)
        .map(|(_, minor, _)| minor)
}

/// Highest allowed semver minor within `major_version` (explicit claim or legacy `minor_ahead` delta).
pub fn licensed_max_super_minor(claims: &LicenseClaims) -> Option<u32> {
    if let Some(max_minor) = claims.max_super_minor {
        return Some(max_minor);
    }
    match (licensed_minor_line(claims), claims.minor_ahead) {
        (Some(issue_minor), Some(ahead)) => Some(issue_minor.saturating_add(ahead)),
        _ => None,
    }
}

/// Human-readable maximum licensed superd version for logs and CLI.
pub fn licensed_version_span(claims: &LicenseClaims) -> String {
    if let Some(issued) = license_issued_for_version(claims) {
        format!("{issued} → max {}", license_max_superd_version(claims))
    } else {
        license_max_superd_version(claims)
    }
}

/// Check whether `superd` semver is within the signed major/minor policy.
pub fn check_superd_version(claims: &LicenseClaims, superd_version: &str) -> Result<(), String> {
    let (host_major, host_minor, _) =
        parse_semver(superd_version).ok_or_else(|| format!("Invalid superd version '{superd_version}'"))?;

    if host_major != claims.major_version {
        return Err(format!(
            "This license is for Super {}.x (max {}). \
             You are running superd {superd_version}. \
             Upgrade your subscription: {LICENSE_UPGRADE_URL}",
            claims.major_version,
            license_max_superd_version(claims),
        ));
    }

    let Some(max_minor) = licensed_max_super_minor(claims) else {
        return Ok(());
    };

    if host_minor > max_minor {
        return Err(format!(
            "This license supports superd up to {host_major}.{max_minor}.x. \
             You are running superd {host_major}.{host_minor}. \
             Upgrade your subscription: {LICENSE_UPGRADE_URL}",
        ));
    }

    Ok(())
}

#[cfg(test)]
mod semver_tests {
    use super::*;

    #[test]
    fn version_labels_are_explicit() {
        let claims = LicenseClaims {
            issued_to: "t".into(),
            issued_at: 1,
            major_version: 1,
            minor_version: Some(2),
            max_super_minor: Some(4),
            minor_ahead: None,
            issued_super_version: Some("1.2.0".into()),
            plugins: vec![],
            expires_at: None,
            retain_plugins_after_expiry: None,
            license_id: None,
        };
        assert_eq!(license_issued_for_version(&claims).as_deref(), Some("1.2.0"));
        assert_eq!(license_max_superd_version(&claims), "1.4.x");
        assert!(superd_within_license(&claims, "1.2.0"));
        assert!(superd_within_license(&claims, "1.4.9"));
        assert!(!superd_within_license(&claims, "1.5.0"));
        assert!(!superd_within_license(&claims, "2.0.0"));
    }

    #[test]
    fn parse_semver_works() {
        assert_eq!(parse_semver("1.2.0"), Some((1, 2, 0)));
        assert_eq!(parse_semver("2"), Some((2, 0, 0)));
    }

    #[test]
    fn issued_super_version_still_parses_minor_for_display() {
        let claims = LicenseClaims {
            issued_to: "t".into(),
            issued_at: 1,
            major_version: 1,
            minor_version: None,
            max_super_minor: Some(4),
            minor_ahead: None,
            issued_super_version: Some("1.2.0".into()),
            plugins: vec![],
            expires_at: None,
            retain_plugins_after_expiry: None,
            license_id: None,
        };
        assert_eq!(licensed_minor_line(&claims), Some(2));
        assert!(check_superd_version(&claims, "1.4.0").is_ok());
        assert!(check_superd_version(&claims, "1.5.0").is_err());
    }

    #[test]
    fn max_super_minor_caps_upper_bound_only() {
        let claims = LicenseClaims {
            issued_to: "t".into(),
            issued_at: 1,
            major_version: 1,
            minor_version: Some(2),
            max_super_minor: Some(4),
            minor_ahead: None,
            issued_super_version: Some("1.2.0".into()),
            plugins: vec![],
            expires_at: None,
            retain_plugins_after_expiry: None,
            license_id: None,
        };
        assert!(check_superd_version(&claims, "1.0.0").is_ok());
        assert!(check_superd_version(&claims, "1.4.0").is_ok());
        assert!(check_superd_version(&claims, "1.5.0").is_err());
        assert!(check_superd_version(&claims, "2.0.0").is_err());
    }

    #[test]
    fn legacy_minor_ahead_delta_still_works() {
        let claims = LicenseClaims {
            issued_to: "t".into(),
            issued_at: 1,
            major_version: 1,
            minor_version: Some(2),
            max_super_minor: None,
            minor_ahead: Some(2),
            issued_super_version: Some("1.2.0".into()),
            plugins: vec![],
            expires_at: None,
            retain_plugins_after_expiry: None,
            license_id: None,
        };
        assert_eq!(licensed_max_super_minor(&claims), Some(4));
        assert_eq!(license_max_superd_version(&claims), "1.4.x");
        assert!(check_superd_version(&claims, "1.4.9").is_ok());
        assert!(check_superd_version(&claims, "1.5.0").is_err());
    }

    #[test]
    fn legacy_license_major_only() {
        let claims = LicenseClaims {
            issued_to: "t".into(),
            issued_at: 1,
            major_version: 1,
            minor_version: None,
            max_super_minor: None,
            minor_ahead: None,
            issued_super_version: None,
            plugins: vec![],
            expires_at: None,
            retain_plugins_after_expiry: None,
            license_id: None,
        };
        assert_eq!(license_max_superd_version(&claims), "1.x");
        assert!(check_superd_version(&claims, "1.9.0").is_ok());
        assert!(check_superd_version(&claims, "2.0.0").is_err());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};
    use rand::rngs::OsRng;

    fn sign_claims(signing_key: &SigningKey, claims: &LicenseClaims) -> String {
        let claims_bytes = serde_json::to_vec(claims).unwrap();
        let signature = signing_key.sign(&claims_bytes);
        let container = LicenseContainer {
            claims: claims.clone(),
            signature: signature.to_bytes().to_vec(),
        };
        BASE64.encode(serde_json::to_vec(&container).unwrap())
    }

    #[test]
    fn parse_major_version_works() {
        assert_eq!(parse_major_version("1.1.9"), 1);
        assert_eq!(parse_major_version("2.0.0"), 2);
    }

    #[test]
    fn plugins_to_features_maps_security_and_isolation() {
        let features = plugins_to_features(&[
            "security".into(),
            "isolation".into(),
            "notify".into(),
        ]);
        assert!(features.contains(&"rbac".to_string()));
        assert!(features.contains(&"audit".to_string()));
        assert!(features.contains(&"cgroups".to_string()));
        assert!(features.contains(&"notify".to_string()));
    }

    #[test]
    fn validate_licensed_plugins_accepts_ui() {
        validate_licensed_plugins(&["ui".into()]).unwrap();
    }

    #[test]
    fn expired_license_still_loads_for_superd() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let claims = LicenseClaims {
            issued_to: "expired@example.com".into(),
            issued_at: 1,
            major_version: 1,
            minor_version: None,
            max_super_minor: None,
            minor_ahead: None,
            issued_super_version: None,
            plugins: vec!["security".into()],
            expires_at: Some(2),
            retain_plugins_after_expiry: Some(true),
            license_id: Some("lic-test".into()),
        };
        let token = sign_claims(&signing_key, &claims);
        let verifying_key = signing_key.verifying_key();
        let (verified, status) =
            verify_license_for_superd_with_key(&token, &verifying_key).expect("superd accepts expired");
        assert_eq!(status, LicenseExpiryStatus::Expired);
        assert_eq!(verified.issued_to, "expired@example.com");
    }

    #[test]
    fn rejects_expired_license() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        let claims = LicenseClaims {
            issued_to: "expired@example.com".into(),
            issued_at: 1,
            major_version: 1,
            minor_version: None,
            max_super_minor: None,
            minor_ahead: None,
            issued_super_version: None,
            plugins: vec!["security".into()],
            expires_at: Some(2),
            retain_plugins_after_expiry: Some(true),
            license_id: Some("lic-test".into()),
        };
        let token = sign_claims(&signing_key, &claims);
        assert!(verify_license_with_key(&token, &verifying_key).is_err());
    }

    #[test]
    fn legacy_claims_without_expiry_still_verify() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        let claims = LicenseClaims {
            issued_to: "legacy".into(),
            issued_at: 1,
            major_version: 1,
            minor_version: None,
            max_super_minor: None,
            minor_ahead: None,
            issued_super_version: None,
            plugins: vec!["security".into()],
            expires_at: None,
            retain_plugins_after_expiry: None,
            license_id: None,
        };
        let token = sign_claims(&signing_key, &claims);
        verify_license_with_key(&token, &verifying_key).expect("legacy license should verify");
    }

    #[test]
    fn superd_rejects_expired_when_claim_says_so() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let claims = LicenseClaims {
            issued_to: "expired@example.com".into(),
            issued_at: 1,
            major_version: 1,
            minor_version: None,
            max_super_minor: None,
            minor_ahead: None,
            issued_super_version: None,
            plugins: vec!["security".into()],
            expires_at: Some(2),
            retain_plugins_after_expiry: Some(false),
            license_id: Some("lic-hard".into()),
        };
        let token = sign_claims(&signing_key, &claims);
        let verifying_key = signing_key.verifying_key();
        assert!(verify_license_for_superd_with_key(&token, &verifying_key).is_err());
    }
}
