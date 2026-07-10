use super::Extension;
use common::{ProgramConfig, SystemEvent};
use std::collections::HashMap;
use uuid::Uuid;

/// Chains multiple extensions; invokes hooks in registration order.
pub struct ExtensionStack {
    layers: Vec<Box<dyn Extension>>,
}

impl ExtensionStack {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn push(&mut self, layer: Box<dyn Extension>) {
        self.layers.push(layer);
    }

    pub fn is_empty(&self) -> bool {
        self.layers.is_empty()
    }
}

impl Default for ExtensionStack {
    fn default() -> Self {
        Self::new()
    }
}

impl Extension for ExtensionStack {
    fn before_start(
        &self,
        id: Uuid,
        config: &ProgramConfig,
    ) -> anyhow::Result<Option<HashMap<String, String>>> {
        let mut merged = HashMap::new();
        for layer in &self.layers {
            if let Some(envs) = layer.before_start(id, config)? {
                merged.extend(envs);
            }
        }
        if merged.is_empty() {
            Ok(None)
        } else {
            Ok(Some(merged))
        }
    }

    fn after_start(&self, id: Uuid, pid: u32, config: &ProgramConfig) -> anyhow::Result<()> {
        for layer in &self.layers {
            layer.after_start(id, pid, config)?;
        }
        Ok(())
    }

    fn before_stop(&self, id: Uuid, config: &ProgramConfig) -> anyhow::Result<()> {
        for layer in &self.layers {
            layer.before_stop(id, config)?;
        }
        Ok(())
    }

    fn after_stop(&self, id: Uuid, config: &ProgramConfig) -> anyhow::Result<()> {
        for layer in &self.layers {
            layer.after_stop(id, config)?;
        }
        Ok(())
    }

    fn on_event(&self, event: SystemEvent) {
        for layer in &self.layers {
            layer.on_event(event.clone());
        }
    }

    fn on_reload(&self) -> anyhow::Result<()> {
        for layer in &self.layers {
            layer.on_reload()?;
        }
        Ok(())
    }

    fn on_shutdown(&self) -> anyhow::Result<()> {
        for layer in self.layers.iter().rev() {
            layer.on_shutdown()?;
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
        for layer in &self.layers {
            layer.on_update(id, pid, old_config, new_config)?;
        }
        Ok(())
    }

    fn collect_metrics(&self) -> String {
        let mut buffer = String::new();
        for layer in &self.layers {
            let metrics = layer.collect_metrics();
            if metrics.is_empty() {
                continue;
            }
            buffer.push_str(&metrics);
            if !metrics.ends_with('\n') {
                buffer.push('\n');
            }
        }
        buffer
    }

    fn supports_resource_limits(&self) -> bool {
        self.layers.iter().any(|layer| layer.supports_resource_limits())
    }
}
