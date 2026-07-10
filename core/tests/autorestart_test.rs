use common::{AutorestartPolicy, ProgramConfig};

#[test]
fn test_autorestart_unexpected_default_exitcodes() {
    let cfg = ProgramConfig {
        autorestart: AutorestartPolicy::Unexpected,
        exitcodes: vec![0],
        ..Default::default()
    };
    assert!(!cfg.should_autorestart(Some(0)));
    assert!(cfg.should_autorestart(Some(1)));
    assert!(cfg.should_autorestart(None));
}

#[test]
fn test_autorestart_true_always_restarts() {
    let cfg = ProgramConfig {
        autorestart: AutorestartPolicy::True,
        ..Default::default()
    };
    assert!(cfg.should_autorestart(Some(0)));
    assert!(cfg.should_autorestart(Some(1)));
}

#[test]
fn test_autorestart_false_never_restarts() {
    let cfg = ProgramConfig {
        autorestart: AutorestartPolicy::False,
        ..Default::default()
    };
    assert!(!cfg.should_autorestart(Some(0)));
    assert!(!cfg.should_autorestart(Some(1)));
}

#[test]
fn test_custom_exitcodes() {
    let cfg = ProgramConfig {
        autorestart: AutorestartPolicy::Unexpected,
        exitcodes: vec![0, 2],
        ..Default::default()
    };
    assert!(!cfg.should_autorestart(Some(2)));
    assert!(cfg.should_autorestart(Some(3)));
}

#[test]
fn test_program_config_serde_defaults() {
    let cfg: ProgramConfig =
        serde_json::from_str(r#"{"name":"a","command":"b","created_at":0,"updated_at":0}"#)
            .unwrap();
    assert_eq!(cfg.autorestart, AutorestartPolicy::Unexpected);
    assert_eq!(cfg.exitcodes, vec![0]);
    assert_eq!(cfg.startsecs, 10);
}
