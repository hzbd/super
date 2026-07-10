use crate::manager::Command;
use common::SystemStats;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use sysinfo::{
    CpuRefreshKind, MemoryRefreshKind, Pid as SysPid, ProcessRefreshKind, ProcessesToUpdate,
    RefreshKind, System,
};
use tokio::sync::mpsc;
use uuid::Uuid;

/// Resource monitor: collects CPU/mem in a background thread and sends to Manager.
pub struct ResourceMonitor {
    pid_mapping: Arc<RwLock<HashMap<Uuid, i32>>>,
    system_stats: Arc<RwLock<SystemStats>>,
}

impl ResourceMonitor {
    pub fn new(tx_manager: mpsc::Sender<Command>) -> Self {
        let pid_mapping = Arc::new(RwLock::new(HashMap::new()));
        let system_stats = Arc::new(RwLock::new(SystemStats::default()));

        let mapping_clone = pid_mapping.clone();
        let stats_clone = system_stats.clone();

        thread::Builder::new()
            .name("super-monitor".to_string())
            .spawn(move || {
                Self::run_loop(mapping_clone, stats_clone, tx_manager);
            })
            .expect("Failed to spawn monitor thread");

        Self {
            pid_mapping,
            system_stats,
        }
    }

    pub fn watch(&self, id: Uuid, pid: u32) {
        if let Ok(mut map) = self.pid_mapping.write() {
            map.insert(id, pid as i32);
        }
    }

    pub fn unwatch(&self, id: &Uuid) {
        if let Ok(mut map) = self.pid_mapping.write() {
            map.remove(id);
        }
    }

    pub fn system_stats(&self) -> SystemStats {
        self.system_stats
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }

    fn run_loop(
        mapping: Arc<RwLock<HashMap<Uuid, i32>>>,
        system_stats: Arc<RwLock<SystemStats>>,
        tx: mpsc::Sender<Command>,
    ) {
        let mut sys = System::new_with_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything())
                .with_processes(ProcessRefreshKind::nothing().with_cpu().with_memory()),
        );

        loop {
            thread::sleep(Duration::from_secs(3));

            sys.refresh_cpu_all();
            sys.refresh_memory();

            if let Ok(mut stats) = system_stats.write() {
                *stats = SystemStats {
                    cpu_percent: sys.global_cpu_usage(),
                    memory_used_bytes: sys.used_memory(),
                    memory_total_bytes: sys.total_memory(),
                    timestamp: chrono::Utc::now().timestamp() as u64,
                };
            }

            let targets: Vec<(Uuid, i32)> = {
                if let Ok(map) = mapping.read() {
                    map.iter().map(|(id, pid)| (*id, *pid)).collect()
                } else {
                    vec![]
                }
            };

            if targets.is_empty() {
                continue;
            }

            let mut updates = HashMap::new();

            let sys_pids: Vec<SysPid> = targets
                .iter()
                .map(|(_, raw)| SysPid::from(*raw as usize))
                .collect();

            sys.refresh_processes(ProcessesToUpdate::Some(&sys_pids), true);

            for (id, raw_pid) in targets {
                let pid = SysPid::from(raw_pid as usize);
                if let Some(proc) = sys.process(pid) {
                    updates.insert(id, (proc.cpu_usage(), proc.memory()));
                }
            }

            if !updates.is_empty() {
                match tx.try_send(Command::InternalMetricsUpdate { metrics: updates }) {
                    Ok(_) => {}
                    Err(tokio::sync::mpsc::error::TrySendError::Closed(_)) => break,
                    Err(tokio::sync::mpsc::error::TrySendError::Full(_)) => {}
                }
            }
        }
    }
}
