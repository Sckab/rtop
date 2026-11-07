use crossterm::event::{KeyCode, KeyEvent};

impl super::App {
    pub fn handle_key_event(&mut self, key: KeyEvent) {
        let (table, length) = if self.show_help {
            (&mut self.states.help_table, self.keys.len())
        } else {
            (&mut self.states.process_table, self.processes.len())
        };

        if self.show_kill_confirmation {
            match key.code {
                KeyCode::Char('y') | KeyCode::Enter => {
                    self.kill_process = true;
                    self.show_kill_confirmation = false;
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Char('q') | KeyCode::Esc => {
                    self.show_kill_confirmation = false;
                }
                _ => {}
            }

            return;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                if self.show_help {
                    self.show_help = false;
                } else {
                    self.quit = true;
                }
            }
            KeyCode::Char('?') | KeyCode::Char('h') => self.show_help = !self.show_help,
            KeyCode::Char('j') | KeyCode::Down => {
                if table.selected().unwrap_or(0) >= length - 1 {
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
            KeyCode::Char('K') => self.show_kill_confirmation = true,
            _ => {}
        }
    }
}
