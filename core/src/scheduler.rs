use std::collections::HashMap;
use std::str::FromStr;
use cron::Schedule;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct CronScheduler {
    // task ID -> (next run time, original cron expression)
    // Keep expression for rescheduling
    tasks: HashMap<Uuid, (DateTime<Utc>, String)>,
}

impl Default for CronScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl CronScheduler {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }

    /// Register or update a cron task
    pub fn upsert(&mut self, id: Uuid, cron_expr: &str) {
        if let Ok(schedule) = Schedule::from_str(cron_expr) {
            if let Some(next) = schedule.upcoming(Utc).next() {
                self.tasks.insert(id, (next, cron_expr.to_string()));
                tracing::debug!("Scheduler: registered {} for {}", id, next);
            }
        } else {
            tracing::warn!("Scheduler: invalid cron expression '{}'", cron_expr);
        }
    }

    /// Remove a task
    pub fn remove(&mut self, id: &Uuid) {
        self.tasks.remove(id);
    }

    /// Return due task IDs and compute next run times
    pub fn tick(&mut self) -> Vec<Uuid> {
        let now = Utc::now();
        let mut triggered = Vec::new();

        // Collect updates separately (avoid simultaneous borrow of self.tasks)
        let mut to_update = Vec::new();

        for (id, (next_run, expr)) in &self.tasks {
            if now >= *next_run {
                triggered.push(*id);
                to_update.push((*id, expr.clone()));
            }
        }

        // Update next run times
        for (id, expr) in to_update {
            if let Ok(schedule) = Schedule::from_str(&expr)
                && let Some(next) = schedule.upcoming(Utc).next()
                    && let Some(entry) = self.tasks.get_mut(&id) {
                        entry.0 = next;
                        tracing::debug!("Scheduler: rescheduled {} for {}", id, next);
                    }
        }

        triggered
    }

    /// Next run time for API display
    pub fn get_next_run(&self, id: &Uuid) -> Option<DateTime<Utc>> {
        self.tasks.get(id).map(|(t, _)| *t)
    }
}
