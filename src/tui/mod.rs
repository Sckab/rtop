use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, widgets::TableState};
use std::{io, sync::mpsc::Receiver};

mod cpu;
mod general;
mod handle_keys;
mod header;
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
    help_table: TableState,
}

pub struct App {
    quit: bool,
    show_help: bool,
    system_info: SystemInfo,
    stats: SystemStats,
    processes: Vec<processes::Process>,
    states: States,
    keys: Vec<help_menu::Key>,
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
            keys: vec![
                help_menu::Key::new("h", "?", "Shows this help panel"),
                help_menu::Key::new("q", "Esc", "Quit the application (or this panel)"),
                help_menu::Key::new("k", "Up", "Select previous process"),
                help_menu::Key::new("j", "Down", "Select next process"),
                help_menu::Key::new("g", "Left", "Select first process"),
                help_menu::Key::new("G", "Right", "Select last process"),
            ],
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let terminal_size = terminal.size()?;

        let rx_stats = workers::create_stats_thread(terminal_size);
        let rx_processes = workers::create_process_thread();

        while !self.quit {
            if self.states.help_table.selected().is_none() {
                self.states.help_table.select_first();
            }

            while let Ok(stats) = rx_stats.try_recv() {
                self.stats = stats;
            }

            while let Ok(processes) = rx_processes.try_recv() {
                let selected_pid = self
                    .states
                    .process_table
                    .selected()
                    .and_then(|index| self.processes.get(index))
                    .map(|process| process.pid);

                self.processes = processes;

                let selected = selected_pid
                    .and_then(|pid| self.processes.iter().position(|process| process.pid == pid))
                    .or_else(|| {
                        if self.processes.is_empty() {
                            None
                        } else {
                            self.states
                                .process_table
                                .selected()
                                .map(|index| index.min(self.processes.len() - 1))
                        }
                    });

                self.states.process_table.select(selected);

                if self.states.process_table.selected().is_none() && !self.processes.is_empty() {
                    self.states.process_table.select_first();
                }
            }

            self.handle_events()?;
            terminal.draw(|frame| self.render(frame.area(), frame.buffer_mut()))?;
        }

        Ok(())
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
