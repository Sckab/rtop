use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::style::Style;
use ratatui_textarea::TextArea;

use super::*;
use crate::app::state::{Mode, SelectedPanel};
use crate::system::process::KillRequest;
use crate::system::sorting::{ProcessSort, ProcessSortType, TemperatureSort, TemperatureSortType};

impl crate::app::App {
    fn handle_sorting(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        let shift = modifiers.contains(KeyModifiers::SHIFT);

        match self.state.selected_panel {
            SelectedPanel::Processes => match key {
                KeyCode::Char('p') | KeyCode::Char('P') => {
                    self.processes_state.sort = ProcessSort::new(ProcessSortType::Pid, shift);
                }
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    self.processes_state.sort = ProcessSort::new(ProcessSortType::Name, shift);
                }
                KeyCode::Char('e') | KeyCode::Char('E') => {
                    self.processes_state.sort = ProcessSort::new(ProcessSortType::Exe, shift);
                }
                KeyCode::Char('u') | KeyCode::Char('U') => {
                    self.processes_state.sort = ProcessSort::new(ProcessSortType::User, shift);
                }
                KeyCode::Char('t') | KeyCode::Char('T') => {
                    self.processes_state.sort = ProcessSort::new(ProcessSortType::Thread, shift);
                }
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    self.processes_state.sort = ProcessSort::new(ProcessSortType::Cpu, shift);
                }
                KeyCode::Char('m') | KeyCode::Char('M') => {
                    self.processes_state.sort = ProcessSort::new(ProcessSortType::Memory, shift);
                }
                KeyCode::Char('r') => {
                    self.processes_state.sort = ProcessSort::new(ProcessSortType::None, shift);
                }
                _ => {}
            },
            SelectedPanel::Termperature => {
                let changed = match key {
                    KeyCode::Char('s') | KeyCode::Char('S') => {
                        self.temperature_state.sort =
                            TemperatureSort::new(TemperatureSortType::Sensor, shift);
                        true
                    }
                    KeyCode::Char('t') | KeyCode::Char('T') => {
                        self.temperature_state.sort =
                            TemperatureSort::new(TemperatureSortType::Temperature, shift);
                        true
                    }
                    KeyCode::Char('r') => {
                        self.temperature_state.sort =
                            TemperatureSort::new(TemperatureSortType::None, shift);
                        true
                    }
                    _ => false,
                };

                if changed {
                    self.resort_temperatures();
                }
            }
        }
    }

    pub(super) fn handle_normal(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.quit = true,
            KeyCode::Char('h') | KeyCode::Char('?') => self.state.mode = Mode::Help,
            KeyCode::Char('K') => {
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
            KeyCode::Char('H') => self.state.selected_panel = SelectedPanel::Termperature,
            KeyCode::Char('L') => self.state.selected_panel = SelectedPanel::Processes,
            code => {
                if key.modifiers.contains(KeyModifiers::ALT) {
                    self.handle_sorting(code, key.modifiers);
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
                    }
                }
            }
        }
    }
}
