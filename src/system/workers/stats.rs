use std::sync::mpsc::Receiver;

use crate::system::{LoadAverage, SystemStats, cpu, history, memory};

pub fn create_worker(
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

            let load_average = match sysinfo::System::load_average() {
                Ok(l) => l,
                Err(_) => sysinfo::LoadAvg::default(),
            };

            while let Ok(w) = rx_width.try_recv() {
                width = w;
            }

            let mut cpus: Vec<cpu::CpuStats> = vec![];

            let mut frequency = 0;

            for (index, cpu) in system.cpus().iter().enumerate() {
                cpus.push(cpu::CpuStats {
                    number: index + 1,
                    usage: cpu.usage(),
                });

                frequency += cpu.frequency();
            }

            frequency = frequency / cpus.len() as u64;

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
                    frequency,
                    brand: system.cpus()[0].brand().to_string(),
                },
                load_average: LoadAverage {
                    one: load_average.one,
                    five: load_average.five,
                    fifteen: load_average.fifteen,
                },
            };

            if tx.send(stats).is_err() {
                break;
            }

            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    });

    rx
}
