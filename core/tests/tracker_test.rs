use super_core::manager::tracker::FlappingTracker;
use uuid::Uuid;

#[test]
fn test_flapping_logic() {
    let mut tracker = FlappingTracker::new();
    let id = Uuid::new_v4();

    // Config: allow 3 restarts within 60 seconds
    let window = 60;
    let threshold = 3;

    // 1. First start
    tracker.record_start(id, threshold);
    assert!(!tracker.is_flapping(id, window, threshold), "1st start should be fine");

    // 2. Second start
    tracker.record_start(id, threshold);
    assert!(!tracker.is_flapping(id, window, threshold), "2nd start should be fine");

    // 3. Third start
    tracker.record_start(id, threshold);
    assert!(!tracker.is_flapping(id, window, threshold), "3rd start should be fine (threshold reached but not exceeded)");

    // 4. Fourth start (should trigger flapping)
    tracker.record_start(id, threshold);
    assert!(tracker.is_flapping(id, window, threshold), "4th start in short time should trigger flapping");
}

#[test]
fn test_flapping_recovery() {
    let mut tracker = FlappingTracker::new();
    let id = Uuid::new_v4();

    // Trigger flapping quickly
    tracker.record_start(id, 1);
    tracker.record_start(id, 1);
    tracker.record_start(id, 1);
    assert!(tracker.is_flapping(id, 60, 1));

    // Reset (simulates manual stop or long elapsed time)
    tracker.reset(&id);

    // Next start should count as the first
    tracker.record_start(id, 1);
    assert!(!tracker.is_flapping(id, 60, 1), "Should be clean after reset");
}
