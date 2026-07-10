use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Linux cgroup resource limits (CPU / memory). Enforced when the isolation plugin is loaded.
#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema)]
pub struct ResourceLimits {
    /// CPU quota percent (50.0 = half a core, 200.0 = two cores).
    #[schema(example = 50.0)]
    pub cpu_quota: Option<f32>,

    /// Memory hard limit in bytes.
    #[schema(example = 104857600)]
    pub memory_limit: Option<u64>,
}
