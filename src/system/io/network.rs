use crate::system::history::History;

#[derive(Default, Debug)]
pub struct Network {
    pub total_download: u64,
    pub total_upload: u64,
    pub download_speed: f64,
    pub upload_speed: f64,
    pub history_rx: History<u64>,
    pub history_tx: History<u64>,
}
