use super_core::logger::{self, LogSource};
use uuid::Uuid;

#[tokio::test]
async fn test_read_log_lines_tail() {
    let dir = tempfile::tempdir().unwrap();
    let id = Uuid::new_v4();
    let path = dir.path().join(format!("{}.out", id));
    let content = (1..=20).map(|i| format!("line-{i}")).collect::<Vec<_>>().join("\n");
    tokio::fs::write(&path, format!("{content}\n")).await.unwrap();

    let tail = logger::read_log_lines(dir.path(), id, LogSource::Stdout, 5, None, None)
        .await
        .unwrap();
    let lines: Vec<&str> = tail.lines().collect();
    assert_eq!(lines.len(), 5);
    assert_eq!(lines[0], "line-16");
    assert_eq!(lines[4], "line-20");
}

#[tokio::test]
async fn test_read_log_lines_custom_path() {
    let dir = tempfile::tempdir().unwrap();
    let id = Uuid::new_v4();
    let custom = dir.path().join("myapp.stdout.log");
    tokio::fs::write(&custom, "custom-log-line\n").await.unwrap();

    let tail = logger::read_log_lines(
        dir.path(),
        id,
        LogSource::Stdout,
        10,
        Some(custom.to_str().unwrap()),
        None,
    )
    .await
    .unwrap();
    assert_eq!(tail.trim(), "custom-log-line");
}
