use common::SystemEvent;
use common::config::EventHookConfig;
use std::sync::{Arc, Mutex};
use super_core::event_hooks;
use uuid::Uuid;

#[tokio::test]
async fn test_event_hook_receives_json_on_stdin() {
    let dir = tempfile::tempdir().unwrap();
    let out_file = dir.path().join("hook.out");
    let out = out_file.display().to_string();

    let script = format!(r#"read line; printf '%s' "$line" > "{out}""#, out = out);

    let hook = EventHookConfig {
        command: script,
        events: vec!["*".to_string()],
        programs: vec!["*".to_string()],
        r#async: false,
        timeout_secs: 5,
        id: Some("test-hook".to_string()),
    };

    let event = SystemEvent::ProcessStarted {
        program_id: Uuid::new_v4(),
        program_name: "demo".to_string(),
        pid: 4242,
    };

    event_hooks::dispatch(&[hook], &event);
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    let body = std::fs::read_to_string(out_file).unwrap();
    assert!(
        body.contains("\"event\":\"process_started\""),
        "body={body}"
    );
    assert!(body.contains("\"name\":\"demo\""), "body={body}");
    assert!(body.contains("4242"), "body={body}");
}

#[derive(Clone)]
struct RecordingExtension {
    events: Arc<Mutex<Vec<SystemEvent>>>,
}

impl super_core::extension::Extension for RecordingExtension {
    fn on_event(&self, event: SystemEvent) {
        self.events.lock().unwrap().push(event);
    }
}

#[tokio::test]
async fn test_emit_notifies_extension_and_runs_hook() {
    let ext = RecordingExtension {
        events: Arc::new(Mutex::new(Vec::new())),
    };
    let extension: Arc<dyn super_core::extension::Extension> = Arc::new(ext.clone());

    let dir = tempfile::tempdir().unwrap();
    let marker = dir.path().join("ran");
    let marker_str = marker.display().to_string();

    let hooks = vec![EventHookConfig {
        command: format!(r#"touch "{}""#, marker_str),
        events: vec!["system_shutdown".to_string()],
        programs: vec!["*".to_string()],
        r#async: false,
        timeout_secs: 5,
        id: None,
    }];

    event_hooks::emit(&extension, &hooks, SystemEvent::SystemShutdown);

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    assert!(marker.exists());
    assert_eq!(ext.events.lock().unwrap().len(), 1);
}
