//! Plugin host: license verification, `plugins/*.so` discovery, and load orchestration.

mod abi;
mod adapter;
mod host;
mod http_host;
mod loader;
mod ui_host;

pub use abi::{PLUGIN_API_VERSION, PLUGIN_SYMBOL, SuperPluginV1};
pub use host::{LicenseOutcome, PluginHost, RunMode};
pub use http_host::attach_http_plugins;
pub use loader::{PluginRuntime, load_authorized_plugins, resolve_plugin_path};
pub use ui_host::{UiPluginHandle, load_ui_plugin, normalize_ui_path};
