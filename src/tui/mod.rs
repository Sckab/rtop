mod cpu;
mod disks;
mod filter;
mod header;
mod help;
mod memory;
mod network;
mod popups;
mod processes;
mod render;
mod temperatures;
mod widgets;

pub const NETWORK_PANEL_PERCENT: u16 = 35;

pub use processes::prepare_display;
