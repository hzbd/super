use common::config::resolve_license_key;
use common::license::{parse_major_version, verify_license, LicenseClaims};
use crate::plugin::loader::{load_authorized_plugins, PluginRuntime};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use tracing::{error, info, warn};

const LICENSE_BANNER: &str = "\
====================================================================\n\
LICENSE ERROR: verification failed. Running OSS mode only.\n\
No paid plugins will be loaded. Check [license].key in conf/super.toml\n\
====================================================================";

/// Whether superd is running with paid plugins active.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunMode {
    Oss,
    Licensed,
}

/// Result of reading and validating the subscription license.
#[derive(Debug, Clone)]
pub enum LicenseOutcome {
    /// No license configured.
    Missing,
    /// Signature valid and major version compatible.
    Valid(LicenseClaims),
    /// Bad signature, parse error, or major mismatch.
    Invalid { reason: String },
}

/// Startup snapshot after scanning license + `plugins/*.so`.
pub struct PluginHost {
    pub mode: RunMode,
    pub claims: Option<LicenseClaims>,
    /// Plugin IDs authorized by license (empty in OSS mode).
    pub licensed_plugins: Vec<String>,
    /// `.so` stems found on disk (all files, including unauthorized).
    pub installed_plugins: Vec<String>,
    /// Plugin IDs loaded successfully via dlopen.
    pub loaded_plugins: Vec<String>,
    pub runtime: PluginRuntime,
    pub plugins_dir: PathBuf,
}

impl PluginHost {
    /// Discover plugins under `{root}/plugins/` and validate license in `conf/super.toml`.
    ///
    /// `superd_version` is used for major-version compatibility checks.
    pub fn discover(root: &Path, superd_version: &str) -> Self {
        let plugins_dir = root.join("plugins");
        let config_file = root.join("conf").join("super.toml");
        let host_major = parse_major_version(superd_version);

        let license_outcome = resolve_license(&config_file);
        let installed_plugins = scan_plugin_files(&plugins_dir);

        match &license_outcome {
            LicenseOutcome::Missing => {
                info!("No license found; running OSS edition.");
                if !installed_plugins.is_empty() {
                    for id in &installed_plugins {
                        warn!(
                            "Plugin '{}.so' present but no license; skipped.",
                            id
                        );
                    }
                }
                return Self {
                    mode: RunMode::Oss,
                    claims: None,
                    licensed_plugins: Vec::new(),
                    installed_plugins,
                    loaded_plugins: Vec::new(),
                    runtime: PluginRuntime::empty(),
                    plugins_dir,
                };
            }
            LicenseOutcome::Invalid { reason } => {
                error!("{}", LICENSE_BANNER);
                error!("License error: {}", reason);
                if !installed_plugins.is_empty() {
                    for id in &installed_plugins {
                        warn!(
                            "Plugin '{}.so' present but license invalid; skipped.",
                            id
                        );
                    }
                }
                return Self {
                    mode: RunMode::Oss,
                    claims: None,
                    licensed_plugins: Vec::new(),
                    installed_plugins,
                    loaded_plugins: Vec::new(),
                    runtime: PluginRuntime::empty(),
                    plugins_dir,
                };
            }
            LicenseOutcome::Valid(claims) => {
                if claims.major_version != host_major {
                    let reason = format!(
                        "License major_version={} incompatible with superd {}.x",
                        claims.major_version, host_major
                    );
                    error!("{}", LICENSE_BANNER);
                    error!("License error: {}", reason);
                    if !installed_plugins.is_empty() {
                        for id in &installed_plugins {
                            warn!(
                                "Plugin '{}.so' present but license invalid; skipped.",
                                id
                            );
                        }
                    }
                    return Self {
                        mode: RunMode::Oss,
                        claims: None,
                        licensed_plugins: Vec::new(),
                        installed_plugins,
                        loaded_plugins: Vec::new(),
                        runtime: PluginRuntime::empty(),
                        plugins_dir,
                    };
                }

                info!(
                    "License verified for '{}' (major v{}, plugins: {:?})",
                    claims.issued_to, claims.major_version, claims.plugins
                );

                let licensed_set: HashSet<&str> =
                    claims.plugins.iter().map(String::as_str).collect();
                let installed_set: HashSet<&str> =
                    installed_plugins.iter().map(String::as_str).collect();

                for id in &installed_plugins {
                    if !licensed_set.contains(id.as_str()) {
                        warn!(
                            "Plugin '{}' present but not licensed; skipped.",
                            id
                        );
                    }
                }

                for id in &claims.plugins {
                    if !installed_set.contains(id.as_str()) {
                        warn!(
                            "Plugin '{}' licensed but not installed; feature unavailable.",
                            id
                        );
                    }
                }

                let to_load: Vec<String> = installed_plugins
                    .iter()
                    .filter(|id| licensed_set.contains(id.as_str()))
                    .cloned()
                    .collect();

                let runtime = load_authorized_plugins(&plugins_dir, &to_load);

                Self {
                    mode: RunMode::Licensed,
                    claims: Some(claims.clone()),
                    licensed_plugins: claims.plugins.clone(),
                    installed_plugins,
                    loaded_plugins: runtime.loaded_ids.clone(),
                    runtime,
                    plugins_dir,
                }
            }
        }
    }

    pub fn is_licensed(&self) -> bool {
        self.mode == RunMode::Licensed
    }

    pub fn has_loaded_plugins(&self) -> bool {
        !self.loaded_plugins.is_empty()
    }
}

fn resolve_license(config_file: &Path) -> LicenseOutcome {
    let key = match resolve_license_key(config_file) {
        Ok(k) => k,
        Err(e) => {
            return LicenseOutcome::Invalid {
                reason: format!("Cannot read license from {:?}: {}", config_file, e),
            };
        }
    };

    let Some(key) = key else {
        return LicenseOutcome::Missing;
    };

    match verify_license(&key) {
        Ok(claims) => LicenseOutcome::Valid(claims),
        Err(e) => LicenseOutcome::Invalid {
            reason: e.to_string(),
        },
    }
}

/// List plugin library stems under `plugins/` (`.so` / `.dylib`).
fn scan_plugin_files(plugins_dir: &Path) -> Vec<String> {
    let mut ids = Vec::new();

    let entries = match std::fs::read_dir(plugins_dir) {
        Ok(e) => e,
        Err(_) => return ids,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let is_plugin_lib = path.extension().is_some_and(|ext| ext == "so" || ext == "dylib");
        if is_plugin_lib
            && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
        {
            ids.push(stem.to_string());
        }
    }

    ids.sort();
    ids.dedup();
    ids
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn scan_finds_so_files() {
        let tmp = TempDir::new().unwrap();
        let plugins = tmp.path().join("plugins");
        std::fs::create_dir_all(&plugins).unwrap();
        std::fs::write(plugins.join("security.so"), b"fake").unwrap();
        std::fs::write(plugins.join("readme.txt"), b"x").unwrap();

        let ids = scan_plugin_files(&plugins);
        assert_eq!(ids, vec!["security"]);
    }

    #[test]
    fn scan_finds_dylib_files() {
        let tmp = TempDir::new().unwrap();
        let plugins = tmp.path().join("plugins");
        std::fs::create_dir_all(&plugins).unwrap();
        std::fs::write(plugins.join("isolation.dylib"), b"fake").unwrap();

        let ids = scan_plugin_files(&plugins);
        assert_eq!(ids, vec!["isolation"]);
    }

    #[test]
    fn missing_license_is_oss() {
        let tmp = TempDir::new().unwrap();
        let host = PluginHost::discover(tmp.path(), "1.1.9");
        assert_eq!(host.mode, RunMode::Oss);
        assert!(host.licensed_plugins.is_empty());
    }

    #[test]
    fn invalid_license_rejects_plugins() {
        let tmp = TempDir::new().unwrap();
        let conf = tmp.path().join("conf");
        std::fs::create_dir_all(&conf).unwrap();
        std::fs::write(
            conf.join("super.toml"),
            "[license]\nkey = \"not-a-license\"\n",
        )
        .unwrap();

        let plugins = tmp.path().join("plugins");
        std::fs::create_dir_all(&plugins).unwrap();
        std::fs::write(plugins.join("notify.so"), b"fake").unwrap();

        let host = PluginHost::discover(tmp.path(), "1.1.9");
        assert_eq!(host.mode, RunMode::Oss);
        assert!(host.loaded_plugins.is_empty());
    }
}
