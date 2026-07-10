use common::ProgramConfig;
use std::collections::HashMap;
use super_core::store;
use tempfile::TempDir;
use uuid::Uuid;

#[tokio::test]
async fn test_persistence() {
    // 1. Set up environment
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("snapshot.json");

    let id1 = Uuid::new_v4();
    let id2 = Uuid::new_v4();

    let mut programs = HashMap::new();

    // 2. Build test data
    let mut config1 = ProgramConfig {
        name: "service-a".to_string(),
        command: "sleep".to_string(),
        args: vec![],
        env: HashMap::new(),
        cwd: None,
        user: None,
        group: None,
        autostart: true,
        retry_limit: 3,
        depends_on: vec!["db".to_string()],
        health_check: None,
        hooks: Default::default(),
        artifact: None,
        created_at: 100,
        updated_at: 200,

        cron: None,
        restore_path: None,

        // Use Default for remaining fields (e.g. resource_limits)
        ..Default::default()
    };
    // Simulate Fatal state (via autostart=false)
    config1.autostart = false;

    programs.insert(id1, config1);
    programs.insert(
        id2,
        ProgramConfig {
            name: "service-b".to_string(),
            command: "echo".to_string(),
            ..programs.get(&id1).unwrap().clone() // Reuse config from id1
        },
    );

    // 3. Write
    store::save(&file_path, &programs)
        .await
        .expect("Save failed");
    assert!(file_path.exists());

    // 4. Read
    let loaded = store::load_with_recovery(&file_path)
        .await
        .expect("Load failed");

    // 5. Verify consistency
    assert_eq!(loaded.len(), 2);
    let loaded_cfg1 = loaded.get(&id1).unwrap();

    assert_eq!(loaded_cfg1.name, "service-a");
    assert!(!loaded_cfg1.autostart); // State should be preserved
    assert_eq!(loaded_cfg1.depends_on, vec!["db".to_string()]);
}
