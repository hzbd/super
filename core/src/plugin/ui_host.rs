//! Static UI bridge for the optional `ui` plugin (`super_plugin_ui_v1`).

use crate::plugin::loader::PluginRuntime;
use common::plugin_ui_abi::{UI_PLUGIN_API_VERSION, UI_PLUGIN_SYMBOL, SuperPluginUiV1};
use libloading::Library;
use std::ffi::{CStr, CString};
use std::sync::Arc;
use tracing::info;

#[derive(Clone)]
pub struct UiPluginHandle {
    resolve_asset: common::plugin_ui_abi::UiResolveAssetFn,
    build_id: Option<common::plugin_ui_abi::UiBuildIdFn>,
}

pub struct UiAsset<'a> {
    pub data: &'a [u8],
    pub mime: &'a str,
}

impl UiPluginHandle {
    pub fn build_id(&self) -> Option<String> {
        let build_id = self.build_id?;
        unsafe {
            let ptr = build_id();
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok().map(str::to_string)
            }
        }
    }

    pub fn resolve(&self, path: &str) -> Option<UiAsset<'_>> {
        let path_c = CString::new(path).ok()?;
        let mut ptr: *const u8 = std::ptr::null();
        let mut len: usize = 0;
        let mut mime_ptr: *const std::ffi::c_char = std::ptr::null();

        let code = unsafe {
            (self.resolve_asset)(
                path_c.as_ptr(),
                &mut ptr,
                &mut len,
                &mut mime_ptr,
            )
        };

        if code != 0 || ptr.is_null() || len == 0 {
            return None;
        }

        let mime = if mime_ptr.is_null() {
            "application/octet-stream"
        } else {
            // SAFETY: plugin returns a NUL-terminated static string.
            unsafe { CStr::from_ptr(mime_ptr).to_str().unwrap_or("application/octet-stream") }
        };

        // SAFETY: pointer/length refer to read-only embedded data in the loaded plugin
        // library; valid until the library is unloaded (never during superd lifetime).
        let data = unsafe { std::slice::from_raw_parts(ptr, len) };
        Some(UiAsset { data, mime })
    }
}

fn load_ui_vtable(library: &Library) -> Option<SuperPluginUiV1> {
    // SAFETY: symbol must match `SuperPluginUiV1` ABI when present.
    unsafe {
        let symbol: Result<libloading::Symbol<unsafe extern "C" fn() -> SuperPluginUiV1>, _> =
            library.get(UI_PLUGIN_SYMBOL);
        symbol.ok().map(|s| s())
    }
}

/// Load the UI plugin vtable when the `ui` plugin is authorized and loaded.
pub fn load_ui_plugin(runtime: &PluginRuntime) -> Option<Arc<UiPluginHandle>> {
    if !runtime.loaded_ids.iter().any(|id| id == "ui") {
        return None;
    }

    let library = runtime.library("ui")?;
    let vtable = load_ui_vtable(library)?;

    if vtable.api_version != UI_PLUGIN_API_VERSION {
        tracing::warn!(
            "UI plugin API version {} != host {}; UI disabled",
            vtable.api_version,
            UI_PLUGIN_API_VERSION
        );
        return None;
    }

    let resolve_asset = vtable.resolve_asset?;

    if let Some(build_id_fn) = vtable.build_id {
        let id = unsafe {
            let ptr = build_id_fn();
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok().map(str::to_string)
            }
        };
        if let Some(id) = id {
            info!("UI plugin loaded (build {})", id);
        } else {
            info!("UI plugin loaded");
        }
    } else {
        info!("UI plugin loaded");
    }

    Some(Arc::new(UiPluginHandle {
        resolve_asset,
        build_id: vtable.build_id,
    }))
}

/// Normalize a request path into a plugin asset key (`index.html`, `assets/app.js`, …).
pub fn normalize_ui_path(uri_path: &str) -> String {
    common::security::sanitize_ui_asset_path(uri_path).unwrap_or_else(|| "index.html".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_root_to_index() {
        assert_eq!(normalize_ui_path("/"), "index.html");
        assert_eq!(normalize_ui_path(""), "index.html");
        assert_eq!(normalize_ui_path("/assets/app.js"), "assets/app.js");
        assert_eq!(normalize_ui_path("/../etc/passwd"), "index.html");
    }
}
