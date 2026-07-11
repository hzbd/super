//! Optional static UI assets exported by the `ui` plugin (`super_plugin_ui_v1`).

pub const UI_PLUGIN_API_VERSION: u32 = 1;
pub const UI_PLUGIN_SYMBOL: &[u8] = b"super_plugin_ui_v1";

/// Resolve a URL path to embedded static bytes.
///
/// Returns:
/// - `0` — found; `*out_ptr` / `*out_len` / `*out_mime` are set (valid for process lifetime)
/// - `1` — not found
/// - `2` — invalid arguments
pub type UiResolveAssetFn = unsafe extern "C" fn(
    path: *const std::ffi::c_char,
    out_ptr: *mut *const u8,
    out_len: *mut usize,
    out_mime: *mut *const std::ffi::c_char,
) -> i32;

/// NUL-terminated build identifier (git sha / package version).
pub type UiBuildIdFn = unsafe extern "C" fn() -> *const std::ffi::c_char;

#[repr(C)]
pub struct SuperPluginUiV1 {
    pub api_version: u32,
    pub resolve_asset: Option<UiResolveAssetFn>,
    pub build_id: Option<UiBuildIdFn>,
}
