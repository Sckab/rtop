use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use std::io;

mod cpu;
mod general;
mod handle_keys;
mod help_menu;
mod history;
mod memory;
mod processes;
mod render;

#[derive(Debug, Default)]
struct SystemStats {
    memory_used: memory::Memory,
    memory_available: memory::Memory,
    memory_free: memory::Memory,
    memory_total: memory::Memory,

    cpu: cpu::Cpu,
}

struct SystemInfo {
    host_name: String,
    os_name: String,
    user_name: String,
    size_formatter: Box<dyn Fn(u64) -> String>,
}

pub struct App {
    quit: bool,
    show_help: bool,
    system_info: SystemInfo,
    stats: SystemStats,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let system_info = SystemInfo {
            host_name: sysinfo::System::host_name()?,
            os_name: sysinfo::System::name()?,
            size_formatter: Box::new(humansize::make_format(humansize::DECIMAL)),

            #[cfg(unix)]
            user_name: std::env::var("USER").unwrap_or("unkown".to_string()),

            #[cfg(windows)]
            user_name: std::env::var("USERNAME").unwrap_or("unkown".to_string()),
        };

        Ok(Self {
            quit: false,
            show_help: false,
            system_info,
            stats: SystemStats::default(),
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let (tx, rx) = std::sync::mpsc::channel::<SystemStats>();

        let terminal_size = terminal.size()?;

        // we need to create a separate thread as the std::thread::sleep() will block the user input events
        std::thread::spawn(move || {
            let mut system = match sysinfo::System::new() {
                Ok(system) => system,
                Err(error) => {
                    eprintln!("Failed to initialize system: {error}");
                    return;
                }
            };

            let width = (terminal_size.width / 2).max(1) as usize;

            let mut cpu_history = history::History::new(width);

            let mut used_memory_history = history::History::new(width);
            let mut available_memory_history = history::History::new(width);
            let mut free_memory_history = history::History::new(width);
            let mut total_memory_history = history::History::new(width);

            loop {
                system.refresh_all();

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

                cpu_history.push(system.global_cpu_usage() as f64);

                used_memory_history.push(used_memory);
                available_memory_history.push(available_memory);
                free_memory_history.push(free_memory);
                total_memory_history.push(total_memory);

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

        while !self.quit {
            while let Ok(stats) = rx.try_recv() {
                self.stats = stats;
            }

            self.handle_events()?;
            terminal.draw(|frame| self.draw(frame))?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event);
                }
                _ => {}
            }
        }

        Ok(())
    }
}
