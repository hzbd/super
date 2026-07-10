use crate::extension::Extension;
use common::plugin_abi::SuperPluginV1;
use common::{ProgramConfig, SystemEvent};
use std::ffi::CStr;
use uuid::Uuid;

type AfterStartFn =
    unsafe extern "C" fn(*const std::ffi::c_char, u32, *const std::ffi::c_char) -> i32;
type AfterStopFn = unsafe extern "C" fn(*const std::ffi::c_char, *const std::ffi::c_char) -> i32;
type OnUpdateFn = unsafe extern "C" fn(
    *const std::ffi::c_char,
    u32,
    *const std::ffi::c_char,
    *const std::ffi::c_char,
) -> i32;
type CollectMetricsFn = unsafe extern "C" fn(*mut std::ffi::c_char, usize) -> usize;
type OnEventFn = unsafe extern "C" fn(*const std::ffi::c_char) -> i32;
type OnReloadFn = unsafe extern "C" fn() -> i32;

/// Adapts a loaded plugin vtable into the in-process `Extension` trait.
pub struct PluginExtensionAdapter {
    name: String,
    after_start: Option<AfterStartFn>,
    after_stop: Option<AfterStopFn>,
    on_update: Option<OnUpdateFn>,
    collect_metrics: Option<CollectMetricsFn>,
    on_event: Option<OnEventFn>,
    on_reload: Option<OnReloadFn>,
}

impl PluginExtensionAdapter {
    pub fn new(expected_id: &str, vtable: SuperPluginV1) -> anyhow::Result<Self> {
        if vtable.api_version != common::plugin_abi::PLUGIN_API_VERSION {
            anyhow::bail!(
                "plugin API version {} != host {}",
                vtable.api_version,
                common::plugin_abi::PLUGIN_API_VERSION
            );
        }

        let plugin_id = unsafe {
            if vtable.plugin_id.is_null() {
                anyhow::bail!("plugin_id is null");
            }
            CStr::from_ptr(vtable.plugin_id)
                .to_str()
                .map_err(|_| anyhow::anyhow!("plugin_id is not valid UTF-8"))?
                .to_string()
        };

        if plugin_id != expected_id {
            anyhow::bail!(
                "plugin id mismatch: file '{}' but library reports '{}'",
                expected_id,
                plugin_id
            );
        }

        if let Some(init) = vtable.init {
            let code = unsafe { init() };
            if code != 0 {
                anyhow::bail!("plugin '{}' init failed with code {}", plugin_id, code);
            }
        }

        Ok(Self {
            name: plugin_id,
            after_start: vtable.after_start,
            after_stop: vtable.after_stop,
            on_update: vtable.on_update,
            collect_metrics: vtable.collect_metrics,
            on_event: vtable.on_event,
            on_reload: vtable.on_reload,
        })
    }
}

impl Extension for PluginExtensionAdapter {
    fn after_start(&self, id: Uuid, pid: u32, config: &ProgramConfig) -> anyhow::Result<()> {
        let Some(hook) = self.after_start else {
            return Ok(());
        };
        let id_c = id.to_string();
        let config_json = serde_json::to_string(config)?;
        let id_ptr = std::ffi::CString::new(id_c)?;
        let cfg_ptr = std::ffi::CString::new(config_json)?;
        let code = unsafe { hook(id_ptr.as_ptr(), pid, cfg_ptr.as_ptr()) };
        if code != 0 {
            anyhow::bail!("plugin '{}' after_start failed ({})", self.name, code);
        }
        Ok(())
    }

    fn after_stop(&self, id: Uuid, config: &ProgramConfig) -> anyhow::Result<()> {
        let Some(hook) = self.after_stop else {
            return Ok(());
        };
        let id_c = id.to_string();
        let config_json = serde_json::to_string(config)?;
        let id_ptr = std::ffi::CString::new(id_c)?;
        let cfg_ptr = std::ffi::CString::new(config_json)?;
        let code = unsafe { hook(id_ptr.as_ptr(), cfg_ptr.as_ptr()) };
        if code != 0 {
            anyhow::bail!("plugin '{}' after_stop failed ({})", self.name, code);
        }
        Ok(())
    }

    fn on_update(
        &self,
        id: Uuid,
        pid: Option<u32>,
        old_config: &ProgramConfig,
        new_config: &ProgramConfig,
    ) -> anyhow::Result<()> {
        let Some(hook) = self.on_update else {
            return Ok(());
        };
        let pid_val = pid.unwrap_or(0);
        let id_c = id.to_string();
        let old_json = serde_json::to_string(old_config)?;
        let new_json = serde_json::to_string(new_config)?;
        let id_ptr = std::ffi::CString::new(id_c)?;
        let old_ptr = std::ffi::CString::new(old_json)?;
        let new_ptr = std::ffi::CString::new(new_json)?;
        let code = unsafe { hook(id_ptr.as_ptr(), pid_val, old_ptr.as_ptr(), new_ptr.as_ptr()) };
        if code != 0 {
            anyhow::bail!("plugin '{}' on_update failed ({})", self.name, code);
        }
        Ok(())
    }

    fn on_event(&self, event: SystemEvent) {
        let Some(hook) = self.on_event else {
            return;
        };
        if let Ok(json) = serde_json::to_string(&event) {
            if let Ok(cstr) = std::ffi::CString::new(json) {
                let _ = unsafe { hook(cstr.as_ptr()) };
            }
        }
    }

    fn on_reload(&self) -> anyhow::Result<()> {
        if let Some(hook) = self.on_reload {
            let code = unsafe { hook() };
            if code != 0 {
                anyhow::bail!("plugin '{}' on_reload failed ({})", self.name, code);
            }
        }
        Ok(())
    }

    fn collect_metrics(&self) -> String {
        let Some(hook) = self.collect_metrics else {
            return String::new();
        };
        let mut buf = vec![0u8; 4096];
        let written = unsafe { hook(buf.as_mut_ptr().cast(), buf.len()) };
        if written == 0 {
            return String::new();
        }
        let end = written.min(buf.len());
        String::from_utf8_lossy(&buf[..end]).into_owned()
    }

    fn supports_resource_limits(&self) -> bool {
        self.name == "isolation"
    }
}
