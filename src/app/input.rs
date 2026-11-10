use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{style::Style, widgets::TableState};
use ratatui_textarea::TextArea;

use super::state::Mode;
use crate::system::process::{KillRequest, Process};

impl super::App {
    pub fn handle_key_event(&mut self, key: KeyEvent) {
        match &mut self.mode {
            Mode::ConfirmKill { request } => match key.code {
                KeyCode::Char('y') | KeyCode::Enter => {
                    self.pending_kill = Some(request.clone());
                    self.mode = Mode::Normal;
                    self.process_query = None;
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
                    let mut text_area =
                        TextArea::new(vec![self.process_query.clone().unwrap_or_default()]);

                    text_area.move_cursor(ratatui_textarea::CursorMove::End);
                    text_area.set_cursor_line_style(Style::default());

                    self.mode = Mode::Search { text_area };
                }
                code => {
                    let length =
                        Self::filtered_processes(self.current_query(), &self.processes).len();

                    navigate(&mut self.tables.process_table, length, code)
                }
            },
            Mode::Search { text_area } => match key.code {
                KeyCode::Esc => {
                    self.mode = Mode::Normal;
                    self.process_query = None;
                }
                KeyCode::Enter => {
                    self.process_query = Some(text_area.lines()[0].clone());
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

        Self::filtered_processes(self.current_query(), &self.processes)
            .get(index)
            .copied()
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
