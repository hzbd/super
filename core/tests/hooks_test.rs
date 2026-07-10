use super_core::hooks;
use std::collections::HashMap;

#[tokio::test]
async fn test_hook_execution() {
    // 1. Basic execution
    let res = hooks::run_hook("echo 'hello world'", &HashMap::new()).await;
    assert!(res.is_ok());
    assert!(res.unwrap()); // Success

    // 2. Script failure
    let res = hooks::run_hook("exit 1", &HashMap::new()).await;
    assert!(res.is_ok());
    assert!(!res.unwrap()); // Failed (Ok with false, not Err — Err means execution error)
}

#[tokio::test]
async fn test_hook_env_injection() {
    let mut envs = HashMap::new();
    envs.insert("SUPER_TEST_VAR".to_string(), "secret_value".to_string());

    // 3. Verify environment variable injection
    // Script exits 0 if the env var is set, otherwise exit 1
    let script = r#"
        if [ "$SUPER_TEST_VAR" = "secret_value" ]; then
            exit 0
        else
            exit 1
        fi
    "#;

    let res = hooks::run_hook(script, &envs).await.unwrap();
    assert!(res, "Hook should detect injected environment variable");
}
