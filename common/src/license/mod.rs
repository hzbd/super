//! Runtime license verification for `superd` (OSS scope).
//!
//! Verifies Ed25519-signed subscription keys and enforces signed claims at
//! runtime. Subscription key signing and plugin catalogs are out of scope
//! for this repository.

mod claims;
mod verify;

pub use claims::{LicenseClaims, LicenseInfo};
pub use verify::{
    check_superd_version, license_expiry_status, license_issued_for_version,
    license_max_superd_version, licensed_max_super_minor, licensed_minor_line,
    licensed_version_span, parse_major_version, parse_semver, superd_within_license,
    verify_license, verify_license_for_superd, LicenseExpiryStatus, LICENSE_UPGRADE_URL,
    PUBLIC_KEY_BYTES,
};
