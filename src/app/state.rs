use ratatui::widgets::TableState;
use std::time::{Duration, Instant};

use crate::system::process::KillRequest;

#[derive(Default)]
pub enum Mode {
    #[default]
    Normal,
    Help,
    Search {
        text_area: ratatui_textarea::TextArea<'static>,
    },
    ConfirmKill {
        request: KillRequest,
    },
}

pub struct Toast {
    pub message: String,
    pub expires_at: Instant,
}

impl Toast {
    pub fn new(message: String) -> Self {
        Self {
            message,
            expires_at: Instant::now() + Duration::from_secs(3),
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at <= Instant::now()
    }
}

pub struct TableStates {
    pub process_table: TableState,
    pub help_table: TableState,
}

impl Default for TableStates {
    fn default() -> Self {
        let mut help_table = TableState::default();
        help_table.select_first();

        Self {
            process_table: TableState::default(),
            help_table,
        }
    }
}
