use anyhow::Context;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use utoipa::ToSchema;

pub const PUBLIC_KEY_BYTES: &[u8] = include_bytes!("../keys/public.key");

/// Signed license claims. The entire struct is covered by Ed25519 signature.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, ToSchema)]
pub struct LicenseClaims {
    pub issued_to: String,
    pub issued_at: u64,
    pub major_version: u32,
    /// Authorized plugin IDs (e.g. `security`, `notify`, `isolation`).
    pub plugins: Vec<String>,
    /// Unix timestamp when the license expires. Omitted = no expiration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u64>,
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
    pub plugins: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license_id: Option<String>,
    /// UI feature codes mapped from authorized plugins.
    pub features: Vec<String>,
}

impl From<&LicenseClaims> for LicenseInfo {
    fn from(claims: &LicenseClaims) -> Self {
        Self {
            issued_to: claims.issued_to.clone(),
            issued_at: claims.issued_at,
            major_version: claims.major_version,
            plugins: claims.plugins.clone(),
            expires_at: claims.expires_at,
            license_id: claims.license_id.clone(),
            features: plugins_to_features(&claims.plugins),
        }
    }
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
    if let Some(expires_at) = claims.expires_at {
        let now = current_unix_secs()?;
        if now > expires_at {
            anyhow::bail!(
                "License expired at {expires_at} (current time {now})"
            );
        }
    }
    Ok(())
}

/// Verify a Base64-encoded signed license string.
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

    ensure_not_expired(&container.claims)?;

    Ok(container.claims)
}

/// Parse the major version from a semver-like string (e.g. "1.1.9" -> 1).
pub fn parse_major_version(version: &str) -> u32 {
    version
        .split('.')
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
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
    fn rejects_expired_license() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        let claims = LicenseClaims {
            issued_to: "expired@example.com".into(),
            issued_at: 1,
            major_version: 1,
            plugins: vec!["security".into()],
            expires_at: Some(2),
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
            plugins: vec!["security".into()],
            expires_at: None,
            license_id: None,
        };
        let token = sign_claims(&signing_key, &claims);
        verify_license_with_key(&token, &verifying_key).expect("legacy license should verify");
    }
}
