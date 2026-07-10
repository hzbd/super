//! Stable C ABI between `superd` and `plugins/*.so` / `*.dylib`.

use std::ffi::CStr;

pub const PLUGIN_API_VERSION: u32 = 1;
pub const PLUGIN_SYMBOL: &[u8] = b"super_plugin_v1";

/// Returns a NUL-terminated semver string (typically `CARGO_PKG_VERSION`).
pub type PluginVersionFn = unsafe extern "C" fn() -> *const std::ffi::c_char;

/// Read the optional release version exported by a lifecycle plugin.
pub fn read_plugin_version(vtable: &SuperPluginV1) -> Option<String> {
    let version_fn = vtable.plugin_version?;
    unsafe {
        let ptr = version_fn();
        if ptr.is_null() {
            None
        } else {
            CStr::from_ptr(ptr).to_str().ok().map(str::to_string)
        }
    }
}

/// Plugin descriptor exported as `super_plugin_v1`.
#[repr(C)]
pub struct SuperPluginV1 {
    pub api_version: u32,
    /// Must match the library filename stem (e.g. `isolation`).
    pub plugin_id: *const std::ffi::c_char,
    /// One-time init. Return 0 on success.
    pub init: Option<unsafe extern "C" fn() -> i32>,
    pub after_start:
        Option<unsafe extern "C" fn(*const std::ffi::c_char, u32, *const std::ffi::c_char) -> i32>,
    pub after_stop:
        Option<unsafe extern "C" fn(*const std::ffi::c_char, *const std::ffi::c_char) -> i32>,
    pub on_update: Option<
        unsafe extern "C" fn(
            *const std::ffi::c_char,
            u32,
            *const std::ffi::c_char,
            *const std::ffi::c_char,
        ) -> i32,
    >,
    /// Writes Prometheus text into `buf`; returns bytes written (excluding NUL).
    pub collect_metrics: Option<unsafe extern "C" fn(*mut std::ffi::c_char, usize) -> usize>,
    /// JSON-encoded `SystemEvent`.
    pub on_event: Option<unsafe extern "C" fn(*const std::ffi::c_char) -> i32>,
    pub on_reload: Option<unsafe extern "C" fn() -> i32>,
    /// Release semver (e.g. `1.2.0`), not the ABI `api_version`.
    pub plugin_version: Option<PluginVersionFn>,
}
