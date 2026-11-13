use crossterm::event::{KeyCode, KeyEvent};

use super::state::Mode;

mod handle_normal;
mod utility;

impl super::App {
    pub fn handle_key_event(&mut self, key: KeyEvent) {
        match &mut self.state.mode {
            Mode::ConfirmKill { request } => match key.code {
                KeyCode::Char('y') | KeyCode::Enter => {
                    self.processes_state.pending_kill = Some(request.clone());
                    self.state.mode = Mode::Normal;
                    self.processes_state.process_query = None;
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Char('q') | KeyCode::Esc => {
                    self.state.mode = Mode::Normal;
                }
                _ => {}
            },
            Mode::Help => match key.code {
                KeyCode::Char('h') | KeyCode::Char('?') | KeyCode::Char('q') | KeyCode::Esc => {
                    self.state.mode = Mode::Normal;
                }
                _ => {}
            },
            Mode::Normal => self.handle_normal(key),
            Mode::Search { text_area } => match key.code {
                KeyCode::Esc => {
                    self.state.mode = Mode::Normal;
                    self.processes_state.process_query = None;
                }
                KeyCode::Enter => {
                    self.processes_state.process_query = Some(text_area.lines()[0].clone());
                    self.state.mode = Mode::Normal;
                }
                _ => {
                    text_area.input(key);
                }
            },
        }
    }
}
