use common::HealthCheck;
use std::time::Duration;
use super_core::health;
use tokio::net::TcpListener;

#[tokio::test]
async fn test_tcp_health_check() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    let check = HealthCheck::Tcp {
        host: "127.0.0.1".to_string(),
        port,
    };

    assert!(
        health::perform_check(&check).await.healthy,
        "TCP check should pass when port is open"
    );

    drop(listener);
    tokio::time::sleep(Duration::from_millis(100)).await;

    let outcome = health::perform_check(&check).await;
    assert!(!outcome.healthy, "TCP check should fail when port is closed");
    assert!(outcome.detail.is_some());
}

#[tokio::test]
async fn test_exec_health_check() {
    let check_ok = HealthCheck::Exec {
        command: "exit 0".to_string(),
    };
    assert!(
        health::perform_check(&check_ok).await.healthy,
        "Exit 0 should be healthy"
    );

    let check_fail = HealthCheck::Exec {
        command: "exit 1".to_string(),
    };
    let outcome = health::perform_check(&check_fail).await;
    assert!(!outcome.healthy, "Exit 1 should be unhealthy");
    assert!(outcome.detail.is_some());
}

#[tokio::test]
async fn test_exec_health_check_reports_stderr() {
    let check = HealthCheck::Exec {
        command: "echo oops 1>&2; exit 1".to_string(),
    };
    let outcome = health::perform_check(&check).await;
    assert!(!outcome.healthy);
    assert!(outcome.detail.unwrap().contains("oops"));
}
