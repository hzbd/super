use anyhow::Context;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
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
}

#[derive(Debug, Serialize, Deserialize)]
struct LicenseContainer {
    claims: LicenseClaims,
    signature: Vec<u8>,
}

/// Verify a Base64-encoded signed license string.
pub fn verify_license(license_str: &str) -> anyhow::Result<LicenseClaims> {
    let json_bytes = BASE64
        .decode(license_str.trim())
        .context("Invalid license format (Base64 decode failed)")?;

    let container: LicenseContainer =
        serde_json::from_slice(&json_bytes).context("Invalid license structure")?;

    let key_array: [u8; 32] = PUBLIC_KEY_BYTES
        .try_into()
        .expect("Embedded public key has incorrect size");

    let verifying_key = VerifyingKey::from_bytes(&key_array)
        .map_err(|_| anyhow::anyhow!("Internal error: invalid embedded public key"))?;

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

    #[test]
    fn parse_major_version_works() {
        assert_eq!(parse_major_version("1.1.9"), 1);
        assert_eq!(parse_major_version("2.0.0"), 2);
    }
}
