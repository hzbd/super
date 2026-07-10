//! Plugin host: license verification, `plugins/*.so` discovery, and load orchestration.

mod abi;
mod adapter;
mod host;
mod http_host;
mod loader;

pub use abi::{PLUGIN_API_VERSION, PLUGIN_SYMBOL, SuperPluginV1};
pub use host::{LicenseOutcome, PluginHost, RunMode};
pub use http_host::attach_http_plugins;
pub use loader::{PluginRuntime, load_authorized_plugins, resolve_plugin_path};
