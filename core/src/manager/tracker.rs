use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

pub struct FlappingTracker {
    // Start history: Uuid -> recent start timestamps (seconds)
    history: HashMap<Uuid, VecDeque<u64>>,
}

impl Default for FlappingTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl FlappingTracker {
    pub fn new() -> Self {
        Self {
            history: HashMap::new(),
        }
    }

    /// Record a start event
    pub fn record_start(&mut self, id: Uuid, threshold: usize) {
        let now = chrono::Utc::now().timestamp() as u64;
        let queue = self.history.entry(id).or_default();

        queue.push_back(now);
        // Keep only the last N+1 entries needed for detection
        while queue.len() > threshold + 1 {
            queue.pop_front();
        }
    }

    /// Detect flapping.
    /// Returns true if restarts within `window` exceed `threshold`.
    pub fn is_flapping(&self, id: Uuid, window: u64, threshold: usize) -> bool {
        if let Some(queue) = self.history.get(&id)
            && queue.len() > threshold {
                let first_time = *queue.front().unwrap();
                let last_time = *queue.back().unwrap(); // simplified: use last recorded start
                // If (Nth start - 1st start) < window, restarts are too frequent
                if last_time - first_time < window {
                    return true;
                }
            }
        false
    }

    /// Clear history (manual stop or reset)
    pub fn reset(&mut self, id: &Uuid) {
        self.history.remove(id);
    }
}
