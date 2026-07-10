use std::thread;
use std::time::Duration;
use super_core::scheduler::CronScheduler;
use uuid::Uuid;

#[test]
fn test_scheduler_logic() {
    let mut scheduler = CronScheduler::new();
    let id = Uuid::new_v4();

    // 1. Register a task that runs every second
    // "* * * * * *" means every second in quartz/cron format (extended)
    // or "0/1 * * * * * *"
    let cron_expr = "0/1 * * * * * *";

    scheduler.upsert(id, cron_expr);

    // Verify next run time was computed
    assert!(
        scheduler.get_next_run(&id).is_some(),
        "Should have next run time"
    );

    // 2. Simulate time passing
    // Should not be due immediately after registration
    let triggered = scheduler.tick();
    assert!(triggered.is_empty(), "Should not trigger immediately");

    println!("⏳ Waiting for cron tick...");
    thread::sleep(Duration::from_secs(2));

    // 3. Check again
    let triggered = scheduler.tick();
    assert_eq!(triggered.len(), 1, "Should trigger after 1 second");
    assert_eq!(triggered[0], id);

    // 4. Verify automatic rescheduling
    let triggered_again = scheduler.tick();
    assert!(
        triggered_again.is_empty(),
        "Should not trigger twice instantly"
    );
    assert!(
        scheduler.get_next_run(&id).is_some(),
        "Should have rescheduled"
    );
}
