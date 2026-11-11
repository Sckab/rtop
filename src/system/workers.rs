use std::path::Path;
use std::sync::mpsc::Receiver;

use crate::system::process::{KillRequest, KillResponse};

use super::{SystemStats, cpu, history, memory, process::Process};

pub fn create_stats_thread(
    rx_width: Receiver<usize>,
    mut system: sysinfo::System,
) -> Receiver<SystemStats> {
    let (tx, rx) = std::sync::mpsc::channel::<SystemStats>();

    // we need to create a separate thread as the std::thread::sleep() will block the user input events
    std::thread::spawn(move || {
        let mut cpu_history = history::History::new();

        let mut used_memory_history = history::History::new();
        let mut available_memory_history = history::History::new();
        let mut free_memory_history = history::History::new();
        let mut total_memory_history = history::History::new();

        let mut width = 0;

        loop {
            system.refresh_memory();
            system.refresh_cpu_all();

            while let Ok(w) = rx_width.try_recv() {
                width = w;
            }

            let mut cpus: Vec<cpu::CpuStats> = vec![];

            for (index, cpu) in system.cpus().iter().enumerate() {
                cpus.push(cpu::CpuStats {
                    number: index + 1,
                    usage: cpu.usage(),
                });
            }

            let used_memory = system.used_memory();
            let available_memory = system.available_memory();
            let free_memory = system.free_memory();
            let total_memory = system.total_memory();

            cpu_history.push(system.global_cpu_usage() as f64, width.saturating_sub(6));

            used_memory_history.push(used_memory, width);
            available_memory_history.push(available_memory, width);
            free_memory_history.push(free_memory, width);
            total_memory_history.push(total_memory, width);

            let stats = SystemStats {
                memory_used: memory::Memory {
                    value: used_memory,
                    history: used_memory_history.clone(),
                },
                memory_available: memory::Memory {
                    value: available_memory,
                    history: available_memory_history.clone(),
                },
                memory_free: memory::Memory {
                    value: free_memory,
                    history: free_memory_history.clone(),
                },
                memory_total: memory::Memory {
                    value: total_memory,
                    history: total_memory_history.clone(),
                },
                cpu: cpu::Cpu {
                    global_usage: system.global_cpu_usage(),
                    cpus,
                    history: cpu_history.clone(),
                },
            };

            if tx.send(stats).is_err() {
                break;
            }

            std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        }
    });

    rx
}

pub fn create_process_thread(
    kill_rx: Receiver<KillRequest>,
    mut system: sysinfo::System,
    users: sysinfo::Users,
) -> (Receiver<Vec<Process>>, Receiver<KillResponse>) {
    let (tx, rx) = std::sync::mpsc::channel();
    let (kill_tx, kill_msg_rx) = std::sync::mpsc::channel::<KillResponse>();

    std::thread::spawn(move || {
        let refresh_kind = sysinfo::ProcessRefreshKind::nothing()
            .with_memory()
            .with_cpu()
            .with_exe(sysinfo::UpdateKind::OnlyIfNotSet)
            .with_tasks()
            .with_user(sysinfo::UpdateKind::OnlyIfNotSet);

        system.refresh_processes_specifics(sysinfo::ProcessesToUpdate::All, true, refresh_kind);

        loop {
            while let Ok(process) = kill_rx.try_recv() {
                if let Some(p) = system.process(sysinfo::Pid::from_u32(process.pid)) {
                    if p.start_time() != process.startup_time || !p.kill() {
                        let _ = kill_tx.send(KillResponse::KillFailed(process.pid));
                    }
                } else {
                    let _ = kill_tx.send(KillResponse::NotFound(process.pid));
                }
            }

            system.refresh_processes_specifics(sysinfo::ProcessesToUpdate::All, true, refresh_kind);

            let processes: Vec<Process> = system
                .processes()
                .iter()
                .filter(|(_, process)| process.thread_kind() != Some(sysinfo::ThreadKind::Userland))
                .map(|(pid, process)| {
                    let user = process
                        .user_id()
                        .and_then(|uid| users.get_user_by_id(uid))
                        .map(|user| user.name().to_string())
                        .unwrap_or_else(|| "Unknown".to_string());

                    Process {
                        pid: pid.as_u32(),
                        name: process.name().to_os_string(),
                        cpu_usage: process.cpu_usage(),
                        memory: process.memory(),
                        exe: process.exe().unwrap_or(Path::new("")).to_path_buf(),
                        threads: process.tasks().map(|tasks| tasks.len() + 1).unwrap_or(1),
                        startup_time: process.start_time(),
                        user,
                        ..Default::default()
                    }
                })
                .collect();

            if tx.send(processes).is_err() {
                break;
            }

            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    });

    (rx, kill_msg_rx)
}
