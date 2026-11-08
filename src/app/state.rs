use ratatui::widgets::TableState;
use std::{
    ffi::OsString,
    time::{Duration, Instant},
};

#[derive(Default)]
pub enum Mode {
    #[default]
    Normal,
    Help,
    ConfirmKill {
        pid: u32,
        name: OsString,
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
    pub(crate) process_table: TableState,
    pub(crate) help_table: TableState,
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
