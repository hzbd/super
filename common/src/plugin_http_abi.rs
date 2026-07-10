//! Optional HTTP hooks exported by plugins (`super_plugin_http_v1`).

pub const HTTP_PLUGIN_API_VERSION: u32 = 1;
pub const HTTP_PLUGIN_SYMBOL: &[u8] = b"super_plugin_http_v1";

/// Initialize plugin HTTP state from JSON: `{ "super_root": "/path/to/install" }`.
pub type HttpInitFn = unsafe extern "C" fn(*const std::ffi::c_char) -> i32;

/// Authenticate a request.
///
/// Returns:
/// - `0` — authenticated (`out_ctx_json` holds opaque user context JSON)
/// - `1` — unauthorized
/// - `2` — server misconfiguration
/// - `3` — whitelist pass (no user context)
pub type HttpAuthFn = unsafe extern "C" fn(
    path: *const std::ffi::c_char,
    authorization: *const std::ffi::c_char,
    query: *const std::ffi::c_char,
    out_ctx_json: *mut std::ffi::c_char,
    out_len: usize,
) -> i32;

/// RBAC check after authentication. Returns `0` allow, `1` forbidden.
pub type HttpRbacFn = unsafe extern "C" fn(
    path: *const std::ffi::c_char,
    method: *const std::ffi::c_char,
    ctx_json: *const std::ffi::c_char,
) -> i32;

/// Record an HTTP mutation audit line.
pub type HttpAuditFn = unsafe extern "C" fn(
    ctx_json: *const std::ffi::c_char,
    method: *const std::ffi::c_char,
    path: *const std::ffi::c_char,
    status: u16,
    client_ip: *const std::ffi::c_char,
);

/// Handle plugin API routes. Returns HTTP status; JSON body in `out` when applicable.
pub type HttpApiFn = unsafe extern "C" fn(
    method: *const std::ffi::c_char,
    path: *const std::ffi::c_char,
    body: *const std::ffi::c_char,
    out: *mut std::ffi::c_char,
    out_len: usize,
) -> u32;

/// Return a JSON route manifest:
/// `[{"method":"GET","path":"/api/..."}, ...]`
pub type HttpListRoutesFn = unsafe extern "C" fn(*mut std::ffi::c_char, usize) -> u32;

#[repr(C)]
pub struct SuperPluginHttpV1 {
    pub api_version: u32,
    pub init: Option<HttpInitFn>,
    pub authenticate: Option<HttpAuthFn>,
    pub authorize: Option<HttpRbacFn>,
    pub audit_request: Option<HttpAuditFn>,
    pub handle_api: Option<HttpApiFn>,
    pub list_routes: Option<HttpListRoutesFn>,
}
