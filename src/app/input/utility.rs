use crossterm::event::KeyCode;
use ratatui::widgets::TableState;

use crate::system::process::Process;

impl crate::app::App {
    pub fn selected_process(&self) -> Option<&Process> {
        let index = self.tables.process_table.selected()?;

        Self::filtered_and_sorted_processes(
            self.current_query(),
            &self.processes_state.processes,
            &self.processes_state.sort,
        )
        .get(index)
        .copied()
    }
}

pub fn navigate(table: &mut TableState, length: usize, key: KeyCode) {
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
