use crate::extension::{Extension, ExtensionStack, NoOpExtension};
use crate::plugin::adapter::PluginExtensionAdapter;
use common::plugin_abi::{PLUGIN_SYMBOL, SuperPluginV1, read_plugin_version};
use libloading::Library;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{error, info};

pub struct PluginRuntime {
    pub loaded_ids: Vec<String>,
    pub plugin_versions: HashMap<String, String>,
    pub extension: Box<dyn Extension>,
    libraries: Vec<(String, Library)>,
}

impl PluginRuntime {
    pub fn empty() -> Self {
        Self {
            loaded_ids: Vec::new(),
            plugin_versions: HashMap::new(),
            extension: Box::new(NoOpExtension),
            libraries: Vec::new(),
        }
    }

    /// Borrow a loaded plugin library by id (for HTTP plugin bridges).
    pub fn library(&self, plugin_id: &str) -> Option<&Library> {
        self.libraries
            .iter()
            .find(|(id, _)| id == plugin_id)
            .map(|(_, lib)| lib)
    }

    /// Take the extension stack for Manager bootstrap while keeping loaded libraries.
    pub fn take_extension(&mut self) -> Box<dyn Extension> {
        std::mem::replace(&mut self.extension, Box::new(NoOpExtension))
    }
}

pub fn load_authorized_plugins(plugins_dir: &Path, authorized_ids: &[String]) -> PluginRuntime {
    let mut stack = ExtensionStack::new();
    let mut libraries = Vec::new();
    let mut loaded_ids = Vec::new();
    let mut plugin_versions = HashMap::new();

    for id in authorized_ids {
        let Some(lib_path) = resolve_plugin_path(plugins_dir, id) else {
            continue;
        };

        match try_load_plugin(&lib_path, id) {
            Ok((library, adapter, version)) => {
                if let Some(version) = version {
                    info!(
                        "Plugin '{}' v{} loaded from {:?}",
                        id, version, lib_path
                    );
                    plugin_versions.insert(id.clone(), version);
                } else {
                    info!("Plugin '{}' loaded from {:?}", id, lib_path);
                }
                stack.push(Box::new(adapter));
                libraries.push((id.clone(), library));
                loaded_ids.push(id.clone());
            }
            Err(e) => {
                error!("Plugin '{}': {}", id, e);
            }
        }
    }

    let extension: Box<dyn Extension> = if stack.is_empty() {
        Box::new(NoOpExtension)
    } else {
        Box::new(stack)
    };

    PluginRuntime {
        loaded_ids,
        plugin_versions,
        extension,
        libraries,
    }
}

fn try_load_plugin(
    lib_path: &Path,
    expected_id: &str,
) -> anyhow::Result<(Library, PluginExtensionAdapter, Option<String>)> {
    // SAFETY: plugin must export `super_plugin_v1` with stable C ABI.
    let library = unsafe { Library::new(lib_path) }?;
    let vtable = unsafe {
        let symbol: libloading::Symbol<unsafe extern "C" fn() -> SuperPluginV1> =
            library.get(PLUGIN_SYMBOL)?;
        symbol()
    };
    let version = read_plugin_version(&vtable);

    let adapter = PluginExtensionAdapter::new(expected_id, vtable)?;
    Ok((library, adapter, version))
}

/// Resolve `plugins/{id}.so` (Linux) or `plugins/{id}.dylib` (macOS).
pub fn resolve_plugin_path(plugins_dir: &Path, id: &str) -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    let extensions = ["dylib", "so"];
    #[cfg(not(target_os = "macos"))]
    let extensions = ["so", "dylib"];

    for ext in extensions {
        let path = plugins_dir.join(format!("{id}.{ext}"));
        if path.is_file() {
            return Some(path);
        }
    }
    None
}
