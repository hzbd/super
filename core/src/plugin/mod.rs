//! Plugin host: license verification, `plugins/*.so` discovery, and load orchestration.

mod abi;
mod adapter;
mod host;
mod http_host;
mod loader;

pub use abi::{SuperPluginV1, PLUGIN_API_VERSION, PLUGIN_SYMBOL};
pub use host::{LicenseOutcome, PluginHost, RunMode};
pub use http_host::attach_http_plugins;
pub use loader::{load_authorized_plugins, PluginRuntime, resolve_plugin_path};
