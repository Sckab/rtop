use crossterm::event::{KeyCode, KeyEvent};

impl super::App {
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                if self.show_help {
                    self.show_help = false
                } else {
                    self.quit = true
                }
            }
            KeyCode::Char('?') | KeyCode::Char('h') => self.show_help = !self.show_help,
            KeyCode::Char('j') | KeyCode::Down => {
                let row = self.states.process_table.selected().unwrap_or_default();

                if row == self.processes.len() - 1 {
                    self.states.process_table.select_first();
                } else {
                    self.states.process_table.select_next();
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                let row = self.states.process_table.selected().unwrap_or_default();

                if row == 0 {
                    self.states.process_table.select_last();
                } else {
                    self.states.process_table.select_previous();
                }
            }
            KeyCode::Char('g') => self.states.process_table.select_first(),
            KeyCode::Char('G') => self.states.process_table.select_last(),
            _ => {}
        }
    }
}
