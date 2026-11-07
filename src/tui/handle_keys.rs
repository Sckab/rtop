use crossterm::event::{KeyCode, KeyEvent};

impl super::App {
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        let (table, length) = if self.show_help {
            (&mut self.states.help_table, self.keys.len())
        } else {
            (&mut self.states.process_table, self.processes.len())
        };

        match key_event.code {
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
            KeyCode::Char('K') => self.kill_process = true,
            _ => {}
        }
    }
}
