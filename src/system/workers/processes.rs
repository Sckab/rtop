use std::path::Path;
use std::sync::mpsc::Receiver;

use crate::system::process::{KillRequest, KillResponse};

use crate::system::process::Process;

pub fn create_worker(
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
