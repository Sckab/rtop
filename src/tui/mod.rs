use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, widgets::TableState};
use std::{io, sync::mpsc::Receiver};

mod cpu;
mod general;
mod handle_keys;
mod help_menu;
mod history;
mod memory;
mod processes;
mod render;
mod workers;

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

#[derive(Default)]
pub struct States {
    process_table: TableState,
}

pub struct App {
    quit: bool,
    show_help: bool,
    system_info: SystemInfo,
    stats: SystemStats,
    processes: Vec<processes::Process>,
    states: States,
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
            processes: vec![],
            states: States::default(),
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let terminal_size = terminal.size()?;

        let rx_stats = workers::create_stats_thread(terminal_size);
        let rx_processes = workers::create_process_thread();

        while !self.quit {
            while let Ok(stats) = rx_stats.try_recv() {
                self.stats = stats;
            }

            while let Ok(processes) = rx_processes.try_recv() {
                self.processes = processes;

                if self.states.process_table.selected().is_none() && !self.processes.is_empty() {
                    self.states.process_table.select_first();
                }
            }

            self.handle_events()?;
            terminal.draw(|frame| self.draw(frame))?;
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        self.render(frame.area(), frame.buffer_mut());
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
