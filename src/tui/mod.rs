use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, widgets::TableState};
use std::{ffi::OsString, io, sync::mpsc::Receiver};

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
    selected_process_pid: u32,
    selected_process_name: OsString,
}

pub struct App {
    quit: bool,
    show_help: bool,
    kill_process: bool,
    show_kill_confirmation: bool,
    show_kill_failed: bool,
    failed_message: String,
    system_info: SystemInfo,
    stats: SystemStats,
    processes: Vec<processes::Process>,
    states: States,
    keys: Vec<help_menu::Key>,
    failed_at: Option<std::time::Instant>,
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
            kill_process: false,
            show_kill_confirmation: false,
            show_kill_failed: false,
            failed_at: None,
            failed_message: "".to_string(),
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
                help_menu::Key::new("K", "", "Kill selected process"),
            ],
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let terminal_size = terminal.size()?;

        let (tx, rx) = std::sync::mpsc::channel::<u32>();

        let rx_stats = workers::create_stats_thread(terminal_size);
        let (rx_processes, rx_msg_kill) = workers::create_process_thread(rx);

        let mut dirty = true;

        while !self.quit {
            if self.states.help_table.selected().is_none() {
                self.states.help_table.select_first();
            }

            if self
                .failed_at
                .is_some_and(|time| time.elapsed() >= std::time::Duration::from_secs(3))
            {
                self.show_kill_failed = false;
                self.failed_at = None;
                dirty = true;
            }

            if self.kill_process {
                let _ = tx.send(self.states.selected_process_pid);
                self.kill_process = false;
            }

            while let Ok(stats) = rx_stats.try_recv() {
                self.stats = stats;
                dirty = true;
            }

            while let Ok(failed) = rx_msg_kill.try_recv() {
                self.show_kill_failed = true;
                self.failed_message = failed;

                self.failed_at = Some(std::time::Instant::now());
                dirty = true;
            }

            while let Ok(mut processes) = rx_processes.try_recv() {
                for process in &mut processes {
                    process.prepare_display(&self.system_info.size_formatter);
                }

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

                dirty = true;
            }

            if self.handle_events()? {
                dirty = true;
            }

            if dirty {
                terminal.draw(|frame| self.render(frame.area(), frame.buffer_mut()))?;
                dirty = false;
            }
        }

        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<bool> {
        if event::poll(std::time::Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event);
                    Ok(true)
                }
                Event::Resize(_, _) => Ok(true),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }
}
