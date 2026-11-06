use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use std::{collections::VecDeque, io};

mod cpu;
mod general;
mod handle_keys;
mod help_menu;
mod memory;
mod processes;
mod render;

#[derive(Debug, Default)]
struct SystemStats {
    memory_used: u64,
    memory_available: u64,
    memory_free: u64,
    memory_total: u64,
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

            let mut history: VecDeque<f64> = VecDeque::new();

            loop {
                system.refresh_all();

                let mut cpus: Vec<cpu::CpuStats> = vec![];

                for (index, cpu) in system.cpus().iter().enumerate() {
                    cpus.push(cpu::CpuStats {
                        number: index + 1,
                        usage: cpu.usage(),
                    });
                }

                if history.len() >= (terminal_size.width / 2) as usize {
                    history.pop_front();
                }

                history.push_back(system.global_cpu_usage() as f64);

                let stats = SystemStats {
                    memory_used: system.used_memory(),
                    memory_available: system.available_memory(),
                    memory_free: system.free_memory(),
                    memory_total: system.total_memory(),
                    cpu: cpu::Cpu {
                        global_usage: system.global_cpu_usage(),
                        cpus,
                        history: history.clone(),
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
