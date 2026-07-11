use common::ProgramConfig;
use std::collections::HashMap;
use super_core::manager::registry::{ProcessRegistry, RuntimeState};
use uuid::Uuid;

// + Helpers +
fn mock_config(name: &str) -> ProgramConfig {
    ProgramConfig {
        name: name.to_string(),
        command: "echo".to_string(),
        args: vec![],
        env: HashMap::new(),
        cwd: None,
        user: None,
        autostart: true,
        retry_limit: 1,
        group: None,
        depends_on: vec![],
        health_check: None,
        hooks: Default::default(),
        artifact: None,
        cron: None,
        created_at: 0,
        updated_at: 0,
        restore_path: None,

        ..Default::default()
    }
}

#[test]
fn test_registry_crud() {
    let mut registry = ProcessRegistry::new(HashMap::new());
    let id = Uuid::new_v4();
    let config = mock_config("test-app");

    // 1. Add config
    registry.programs.insert(id, config.clone());
    assert!(registry.get_config(&id).is_some());

    // 2. Dirty flag
    assert!(!registry.dirty);
    registry.mark_dirty();
    assert!(registry.dirty);

    // 3. Runtime state
    // Note: RuntimeState has no is_upgrading field;
    // the design tracks transactions via restore_path in ProgramConfig.
    registry.running.insert(
        id,
        RuntimeState {
            pid: 1234,
            start_time: 100,
            retry_count: 0,
            stopping: false,
            restart_requested: false,
            is_healthy: true,
            health_error: None,
            health_task: None,
            alert_pending_recovery: false,
            cpu_usage: 0.0,
            mem_usage: 0,
        },
    );

    let state = registry.get_running(&id).unwrap();
    assert_eq!(state.pid, 1234);
    assert!(state.is_healthy);

    // 4. Remove
    registry.running.remove(&id);
    registry.programs.remove(&id);
    assert!(registry.get_config(&id).is_none());
}
