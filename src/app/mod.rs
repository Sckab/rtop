use crossterm::event::{self, Event, KeyEventKind};
use ratatui::DefaultTerminal;
use std::io;

use crate::system::{
    SystemInfo, SystemStats,
    process::{KillRequest, KillResponse, ProcessesState},
    workers,
};

mod input;
pub mod keymap;
pub mod state;

use state::{Mode, TableStates, Toast};

pub struct App {
    pub quit: bool,
    pub mode: Mode,
    pub toast: Option<Toast>,
    pub system_info: SystemInfo,
    pub stats: SystemStats,
    pub tables: TableStates, // we still leave this, it could be useful in the future
    pub keys: keymap::Keys,
    pub processes_state: ProcessesState,
}

impl App {
    pub fn new() -> Self {
        let system_info = SystemInfo {
            host_name: whoami::hostname().unwrap_or_else(|_| "Unknown".to_string()),
            os_name: whoami::distro().unwrap_or_else(|_| "Unknown".to_string()),
            user_name: whoami::username().unwrap_or_else(|_| "Unknown".to_string()),
            size_formatter: Box::new(humansize::make_format(humansize::DECIMAL)),
        };

        Self {
            quit: false,
            mode: Mode::Normal,
            toast: None,
            system_info,
            stats: SystemStats::default(),
            tables: TableStates::default(),
            keys: keymap::Keys::new(),
            processes_state: ProcessesState::default(),
        }
    }

    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        stats_system: sysinfo::System,
        process_system: sysinfo::System,
        users: sysinfo::Users,
    ) -> io::Result<()> {
        let (tx_kill, rx_kill) = std::sync::mpsc::channel::<KillRequest>();
        let (tx_width, rx_width) = std::sync::mpsc::channel::<usize>();

        let _ = tx_width.send((terminal.size()?.width / 2) as usize);

        let rx_stats = workers::create_stats_thread(rx_width, stats_system);
        let (rx_processes, rx_msg_kill) =
            workers::create_process_thread(rx_kill, process_system, users);

        let mut dirty = true;

        while !self.quit {
            let _ = tx_width.send((terminal.size()?.width / 2) as usize);

            if self.toast.as_ref().is_some_and(Toast::is_expired) {
                self.toast = None;
                dirty = true;
            }

            if let Some(pid) = self.processes_state.pending_kill.take() {
                let _ = tx_kill.send(pid);
            }

            while let Ok(stats) = rx_stats.try_recv() {
                self.stats = stats;
                dirty = true;
            }

            while let Ok(response) = rx_msg_kill.try_recv() {
                let message = match response {
                    // This shouldn't happen, but the process may have already exited
                    // by the time the user confirms the kill request.
                    KillResponse::NotFound(pid) => {
                        format!("{pid} not found.")
                    }
                    KillResponse::KillFailed(pid) => {
                        format!("Failed to kill {pid}")
                    }
                };

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
                    .and_then(|index| self.processes_state.processes.get(index))
                    .map(|process| process.pid);

                self.processes_state.processes = processes;

                let selected = selected_pid
                    .and_then(|pid| {
                        self.processes_state
                            .processes
                            .iter()
                            .position(|process| process.pid == pid)
                    })
                    .or_else(|| {
                        if self.processes_state.processes.is_empty() {
                            None
                        } else {
                            self.tables
                                .process_table
                                .selected()
                                .map(|index| index.min(self.processes_state.processes.len() - 1))
                        }
                    });

                self.tables.process_table.select(selected);

                if self.tables.process_table.selected().is_none()
                    && !self.processes_state.processes.is_empty()
                {
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
        if event::poll(std::time::Duration::from_millis(100))? {
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
