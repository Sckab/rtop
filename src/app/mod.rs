use crossterm::event::{self, Event, KeyEventKind};
use ratatui::DefaultTerminal;
use std::io;

use crate::system::{SystemInfo, SystemStats, process::Process, workers};

mod input;
pub(crate) mod keymap;
pub(crate) mod state;

use state::{Mode, TableStates, Toast};

pub struct App {
    pub(crate) quit: bool,
    pub(crate) mode: Mode,
    pub(crate) pending_kill: Option<u32>,
    pub(crate) toast: Option<Toast>,
    pub(crate) system_info: SystemInfo,
    pub(crate) stats: SystemStats,
    pub(crate) processes: Vec<Process>,
    pub(crate) tables: TableStates,
    pub(crate) keys: Vec<keymap::Key>,
    pub(crate) process_query: Option<String>,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let system_info = SystemInfo {
            host_name: sysinfo::System::host_name()?,
            os_name: sysinfo::System::name()?,
            size_formatter: Box::new(humansize::make_format(humansize::DECIMAL)),

            #[cfg(unix)]
            user_name: std::env::var("USER").unwrap_or("unknown".to_string()),

            #[cfg(windows)]
            user_name: std::env::var("USERNAME").unwrap_or("unknown".to_string()),
        };

        Ok(Self {
            quit: false,
            mode: Mode::Normal,
            pending_kill: None,
            toast: None,
            system_info,
            stats: SystemStats::default(),
            processes: vec![],
            tables: TableStates::default(),
            keys: keymap::default_keys(),
            process_query: None,
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let terminal_size = terminal.size()?;

        let (tx, rx) = std::sync::mpsc::channel::<u32>();

        let rx_stats = workers::create_stats_thread(terminal_size.width);
        let (rx_processes, rx_msg_kill) = workers::create_process_thread(rx);

        let mut dirty = true;

        while !self.quit {
            if self.toast.as_ref().is_some_and(Toast::is_expired) {
                self.toast = None;
                dirty = true;
            }

            if let Some(pid) = self.pending_kill.take() {
                let _ = tx.send(pid);
            }

            while let Ok(stats) = rx_stats.try_recv() {
                self.stats = stats;
                dirty = true;
            }

            while let Ok(message) = rx_msg_kill.try_recv() {
                self.toast = Some(Toast::new(message));
                dirty = true;
            }

            while let Ok(mut processes) = rx_processes.try_recv() {
                for process in &mut processes {
                    crate::tui::prepare_display(process, &self.system_info.size_formatter);
                }

                let selected_pid = self
                    .tables
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
                            self.tables
                                .process_table
                                .selected()
                                .map(|index| index.min(self.processes.len() - 1))
                        }
                    });

                self.tables.process_table.select(selected);

                if self.tables.process_table.selected().is_none() && !self.processes.is_empty() {
                    self.tables.process_table.select_first();
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
