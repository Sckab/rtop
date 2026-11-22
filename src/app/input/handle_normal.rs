use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::style::Style;
use ratatui_textarea::TextArea;

use super::*;
use crate::app::state::{Mode, SelectedPanel};
use crate::system::process::KillRequest;
use crate::system::sorting::{
    DisksSort, DisksSortType, ProcessSort, ProcessSortType, TemperatureSort, TemperatureSortType,
};

impl crate::app::App {
    fn handle_panel_keys(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        let shift = modifiers.contains(KeyModifiers::SHIFT);

        match self.state.selected_panel {
            SelectedPanel::Processes => match key {
                KeyCode::Char('p') | KeyCode::Char('P') => {
                    self.sortings.processes = ProcessSort::new(ProcessSortType::Pid, shift);
                }
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    self.sortings.processes = ProcessSort::new(ProcessSortType::Name, shift);
                }
                KeyCode::Char('e') | KeyCode::Char('E') => {
                    self.sortings.processes = ProcessSort::new(ProcessSortType::Exe, shift);
                }
                KeyCode::Char('u') | KeyCode::Char('U') => {
                    self.sortings.processes = ProcessSort::new(ProcessSortType::User, shift);
                }
                KeyCode::Char('t') | KeyCode::Char('T') => {
                    self.sortings.processes = ProcessSort::new(ProcessSortType::Thread, shift);
                }
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    self.sortings.processes = ProcessSort::new(ProcessSortType::Cpu, shift);
                }
                KeyCode::Char('m') | KeyCode::Char('M') => {
                    self.sortings.processes = ProcessSort::new(ProcessSortType::Memory, shift);
                }
                KeyCode::Char('r') => {
                    self.sortings.processes = ProcessSort::new(ProcessSortType::None, shift);
                }
                KeyCode::Char('D') => {
                    if let Some(process) = self.selected_process() {
                        self.state.mode = Mode::ConfirmKill {
                            request: KillRequest {
                                pid: process.pid,
                                name: process.name.clone(),
                                startup_time: process.startup_time,
                            },
                        };
                    }
                }
                _ => {}
            },
            SelectedPanel::Termperature => {
                let changed = match key {
                    KeyCode::Char('s') | KeyCode::Char('S') => {
                        self.sortings.temperatures =
                            TemperatureSort::new(TemperatureSortType::Sensor, shift);
                        true
                    }
                    KeyCode::Char('t') | KeyCode::Char('T') => {
                        self.sortings.temperatures =
                            TemperatureSort::new(TemperatureSortType::Temperature, shift);
                        true
                    }
                    KeyCode::Char('r') => {
                        self.sortings.temperatures =
                            TemperatureSort::new(TemperatureSortType::None, shift);
                        true
                    }
                    _ => false,
                };

                if changed {
                    self.resort_temperatures();
                }
            }
            SelectedPanel::DiskUsage => {
                let changed = match key {
                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        self.sortings.disks = DisksSort::new(DisksSortType::Name, shift);
                        true
                    }
                    KeyCode::Char('m') | KeyCode::Char('M') => {
                        self.sortings.disks = DisksSort::new(DisksSortType::MountPoint, shift);
                        true
                    }
                    KeyCode::Char('u') | KeyCode::Char('U') => {
                        self.sortings.disks = DisksSort::new(DisksSortType::Used, shift);
                        true
                    }
                    KeyCode::Char('a') | KeyCode::Char('A') => {
                        self.sortings.disks = DisksSort::new(DisksSortType::Available, shift);
                        true
                    }
                    KeyCode::Char('t') | KeyCode::Char('T') => {
                        self.sortings.disks = DisksSort::new(DisksSortType::Total, shift);
                        true
                    }
                    KeyCode::Char('d') | KeyCode::Char('D') => {
                        self.sortings.disks = DisksSort::new(DisksSortType::Read, shift);
                        true
                    }
                    KeyCode::Char('w') | KeyCode::Char('W') => {
                        self.sortings.disks = DisksSort::new(DisksSortType::Write, shift);
                        true
                    }
                    KeyCode::Char('r') => {
                        self.sortings.disks = DisksSort::new(DisksSortType::None, shift);
                        true
                    }
                    _ => false,
                };

                if changed {
                    self.resort_disks();
                }
            }
        }
    }

    pub(super) fn handle_normal(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.quit = true,
            KeyCode::Char('h') | KeyCode::Char('?') => self.state.mode = Mode::Help,
            KeyCode::Char('/') => {
                let mut text_area = TextArea::new(vec![
                    self.processes_state
                        .process_query
                        .clone()
                        .unwrap_or_default(),
                ]);

                text_area.move_cursor(ratatui_textarea::CursorMove::End);
                text_area.set_cursor_line_style(Style::default());

                self.state.mode = Mode::Search { text_area };
            }
            KeyCode::Char('K') => self.state.selected_panel = SelectedPanel::Processes,
            KeyCode::Char('J') => {
                if let SelectedPanel::Processes = self.state.selected_panel {
                    self.state.selected_panel = SelectedPanel::DiskUsage;
                }
            }
            KeyCode::Char('H') => {
                if let SelectedPanel::DiskUsage = self.state.selected_panel {
                    self.state.selected_panel = SelectedPanel::Termperature;
                }
            }
            KeyCode::Char('L') => {
                if let SelectedPanel::Termperature = self.state.selected_panel {
                    self.state.selected_panel = SelectedPanel::DiskUsage;
                }
            }
            code => {
                if key.modifiers.contains(KeyModifiers::ALT) {
                    self.handle_panel_keys(code, key.modifiers);
                } else {
                    match self.state.selected_panel {
                        SelectedPanel::Processes => {
                            let length = Self::filtered_processes(
                                self.current_query(),
                                &self.processes_state.processes,
                            )
                            .len();

                            utility::navigate(&mut self.tables.process_table, length, code);
                        }
                        SelectedPanel::Termperature => {
                            utility::navigate(
                                &mut self.tables.temperature_table,
                                self.temperature_state.temperatures.len(),
                                code,
                            );
                        }
                        SelectedPanel::DiskUsage => {
                            utility::navigate(
                                &mut self.tables.disks_table,
                                self.io.disks.disks.len(),
                                code,
                            );
                        }
                    }
                }
            }
        }
    }
}
