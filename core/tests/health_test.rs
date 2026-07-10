use common::HealthCheck;
use std::time::Duration;
use super_core::health;
use tokio::net::TcpListener;

#[tokio::test]
async fn test_tcp_health_check() {
    // 1. Start a listener on a random port (simulates a healthy service)
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    let check = HealthCheck::Tcp {
        host: "127.0.0.1".to_string(),
        port,
    };

    // 2. Should succeed
    assert!(
        health::perform_check(&check).await,
        "TCP check should pass when port is open"
    );

    // 3. Close listener (simulates service down)
    drop(listener);
    // Allow time for the port to be reclaimed
    tokio::time::sleep(Duration::from_millis(100)).await;

    // 4. Should fail
    assert!(
        !health::perform_check(&check).await,
        "TCP check should fail when port is closed"
    );
}

#[tokio::test]
async fn test_exec_health_check() {
    // 1. Successful command
    let check_ok = HealthCheck::Exec {
        command: "exit 0".to_string(),
    };
    assert!(
        health::perform_check(&check_ok).await,
        "Exit 0 should be healthy"
    );

    // 2. Failing command
    let check_fail = HealthCheck::Exec {
        command: "exit 1".to_string(),
    };
    assert!(
        !health::perform_check(&check_fail).await,
        "Exit 1 should be unhealthy"
    );
}
