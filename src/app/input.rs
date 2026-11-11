use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{style::Style, widgets::TableState};
use ratatui_textarea::TextArea;

use super::state::Mode;
use crate::system::process::{KillRequest, Process, Sort, SortType};

impl super::App {
    pub fn handle_key_event(&mut self, key: KeyEvent) {
        match &mut self.mode {
            Mode::ConfirmKill { request } => match key.code {
                KeyCode::Char('y') | KeyCode::Enter => {
                    self.processes_state.pending_kill = Some(request.clone());
                    self.mode = Mode::Normal;
                    self.processes_state.process_query = None;
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Char('q') | KeyCode::Esc => {
                    self.mode = Mode::Normal;
                }
                _ => {}
            },
            Mode::Help => match key.code {
                KeyCode::Char('h') | KeyCode::Char('?') | KeyCode::Char('q') | KeyCode::Esc => {
                    self.mode = Mode::Normal;
                }
                code => navigate(&mut self.tables.help_table, self.keys.len(), code),
            },
            Mode::Normal => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.quit = true,
                KeyCode::Char('h') | KeyCode::Char('?') => self.mode = Mode::Help,
                KeyCode::Char('K') => {
                    if let Some(process) = self.selected_process() {
                        self.mode = Mode::ConfirmKill {
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

                    self.mode = Mode::Search { text_area };
                }
                code => {
                    if key.modifiers.contains(KeyModifiers::ALT) {
                        match code {
                            KeyCode::Char('p') | KeyCode::Char('P') => {
                                self.processes_state.sort = Sort::Pid(get_sort_type(key.modifiers));
                            }
                            KeyCode::Char('n') | KeyCode::Char('N') => {
                                self.processes_state.sort =
                                    Sort::Name(get_sort_type(key.modifiers));
                            }
                            KeyCode::Char('e') | KeyCode::Char('E') => {
                                self.processes_state.sort = Sort::Exe(get_sort_type(key.modifiers));
                            }
                            KeyCode::Char('u') | KeyCode::Char('U') => {
                                self.processes_state.sort =
                                    Sort::User(get_sort_type(key.modifiers));
                            }
                            KeyCode::Char('t') | KeyCode::Char('T') => {
                                self.processes_state.sort =
                                    Sort::Thread(get_sort_type(key.modifiers));
                            }
                            KeyCode::Char('c') | KeyCode::Char('C') => {
                                self.processes_state.sort = Sort::Cpu(get_sort_type(key.modifiers));
                            }
                            KeyCode::Char('m') | KeyCode::Char('M') => {
                                self.processes_state.sort =
                                    Sort::Memory(get_sort_type(key.modifiers));
                            }
                            KeyCode::Char('r') => {
                                self.processes_state.sort = Sort::None;
                            }
                            _ => {}
                        }
                    } else {
                        let length = Self::filtered_processes(
                            self.current_query(),
                            &self.processes_state.processes,
                        )
                        .len();

                        navigate(&mut self.tables.process_table, length, code)
                    }
                }
            },
            Mode::Search { text_area } => match key.code {
                KeyCode::Esc => {
                    self.mode = Mode::Normal;
                    self.processes_state.process_query = None;
                }
                KeyCode::Enter => {
                    self.processes_state.process_query = Some(text_area.lines()[0].clone());
                    self.mode = Mode::Normal;
                }
                _ => {
                    text_area.input(key);
                }
            },
        }
    }

    fn selected_process(&self) -> Option<&Process> {
        let index = self.tables.process_table.selected()?;

        Self::filtered_processes(self.current_query(), &self.processes_state.processes)
            .get(index)
            .copied()
    }
}

fn get_sort_type(modifier: KeyModifiers) -> SortType {
    if modifier.contains(KeyModifiers::SHIFT) {
        SortType::Descending
    } else {
        SortType::Ascending
    }
}

fn navigate(table: &mut TableState, length: usize, key: KeyCode) {
    if length == 0 {
        return;
    }

    match key {
        KeyCode::Char('j') | KeyCode::Down => {
            if table.selected().unwrap_or(0).saturating_add(1) >= length {
                table.select_first();
            } else {
                table.select_next();
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            if table.selected().unwrap_or(0) == 0 {
                table.select_last();
            } else {
                table.select_previous();
            }
        }
        KeyCode::Char('g') | KeyCode::Left => table.select_first(),
        KeyCode::Char('G') | KeyCode::Right => table.select_last(),
        _ => {}
    }
}
