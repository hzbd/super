use common::{ProgramConfig, SystemEvent};
use std::collections::HashMap;
use uuid::Uuid;

/// Core extension interface.
/// Middleware-style hooks invoked at key lifecycle points.
pub trait Extension: Send + Sync {
    // 1. Start interception
    /// [Pre-Start] Preparation phase.
    /// Timing: before process spawn.
    /// Default: no extra env; allow start.
    fn before_start(
        &self,
        _id: Uuid,
        _config: &ProgramConfig,
    ) -> anyhow::Result<Option<HashMap<String, String>>> {
        Ok(None)
    }

    /// [Post-Start] Apply phase.
    /// Timing: immediately after PID is assigned.
    /// Default: OSS build applies no limits.
    fn after_start(&self, _id: Uuid, _pid: u32, _config: &ProgramConfig) -> anyhow::Result<()> {
        Ok(())
    }

    // 2. Stop interception
    /// [Pre-Stop] Drain phase.
    /// Timing: before sending stop signal.
    /// Default: OSS build has nothing to drain.
    fn before_stop(&self, _id: Uuid, _config: &ProgramConfig) -> anyhow::Result<()> {
        Ok(())
    }

    /// [Post-Stop] Cleanup phase.
    /// Timing: after process exit.
    /// Default: OSS build has no resources to release.
    fn after_stop(&self, _id: Uuid, _config: &ProgramConfig) -> anyhow::Result<()> {
        Ok(())
    }

    // 3. System capabilities
    /// [Event] Observe system events.
    /// Default: OSS build ignores all events.
    fn on_event(&self, _event: SystemEvent) {}

    /// [Reload] Config reload hook.
    fn on_reload(&self) -> anyhow::Result<()> {
        Ok(())
    }

    /// [Shutdown] Graceful shutdown hook.
    fn on_shutdown(&self) -> anyhow::Result<()> {
        Ok(())
    }

    // Config update hook: fired when program config changes.
    // old_config: before update
    // new_config: after update
    // pid: running process PID, or None if stopped
    fn on_update(
        &self,
        _id: Uuid,
        _pid: Option<u32>,
        _old_config: &ProgramConfig,
        _new_config: &ProgramConfig,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    // 4. Observability
    /// Prometheus metrics from the extension.
    /// Output must follow Prometheus exporter format.
    fn collect_metrics(&self) -> String {
        String::new() // default: empty
    }

    /// Whether this extension enforces `ProgramConfig::resource_limits` (e.g. isolation plugin).
    fn supports_resource_limits(&self) -> bool {
        false
    }
}

// OSS default no-op extension (injected by superd entrypoint)
pub struct NoOpExtension;

impl Extension for NoOpExtension {}

mod stack;
pub use stack::ExtensionStack;
