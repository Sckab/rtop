use ratatui::widgets::TableState;
use std::time::{Duration, Instant};

use crate::system::process::KillRequest;

// A single `Mode` is stored in `App`, so boxing the large `TextArea` variant
// would only add an allocation and indirection without a measurable benefit.
#[derive(Default)]
#[allow(clippy::large_enum_variant)]
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

#[derive(Default)]
pub enum SelectedPanel {
    #[default]
    Processes,
    Termperature,
}

#[derive(Default)]
pub struct State {
    pub selected_panel: SelectedPanel,
    pub mode: Mode,
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

#[derive(Default)]
pub struct TableStates {
    pub process_table: TableState,
    pub temperature_table: TableState,
}
