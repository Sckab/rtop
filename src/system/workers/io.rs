use std::{sync::mpsc::Receiver, time::Instant};

use regex::Regex;

use crate::system::{
    history,
    io::{self, Io, disks, network},
};

pub fn create_worker(
    rx_width: Receiver<usize>,
    mut network_info: sysinfo::Networks,
    mut disks_info: sysinfo::Disks,
) -> Receiver<Io> {
    let (tx, rx) = std::sync::mpsc::channel::<Io>();

    std::thread::spawn(move || {
        let mut download_history = history::History::new();
        let mut upload_history = history::History::new();

        let mut width = 0;

        let excluded_interfaces: Vec<Regex> = io::EXCLUDED_NETWORK_INTERFACES
            .iter()
            .filter_map(|pattern| Regex::new(pattern).ok())
            .collect();

        let mut last_refresh: Option<Instant> = None;

        disks_info.refresh(true);
        network_info.refresh(true);

        loop {
            disks_info.refresh(true);
            network_info.refresh(true);

            let elapsed = last_refresh.map(|instant| instant.elapsed().as_secs_f64());
            last_refresh = Some(Instant::now());

            while let Ok(w) = rx_width.try_recv() {
                width = w;
            }

            let mut disks = vec![];

            for disk in &disks_info {
                let usage = disk.usage();

                disks.push(disks::Disk {
                    name: disk.name().to_os_string(),
                    mount: disk.mount_point().to_path_buf(),
                    read: usage.read_bytes,
                    write: usage.written_bytes,
                    available: disk.available_space(),
                    total: disk.total_space(),
                });
            }

            let mut download = 0;
            let mut upload = 0;

            let mut total_download = 0;
            let mut total_upload = 0;

            for (interface, network) in &network_info {
                if excluded_interfaces
                    .iter()
                    .any(|regex| regex.is_match(&interface.to_string_lossy()))
                {
                    continue;
                }

                download += network.received();
                upload += network.transmitted();

                total_download += network.total_received();
                total_upload += network.total_transmitted();
            }

            let (download_speed, upload_speed) = match elapsed {
                Some(elapsed) if elapsed > 0.0 => {
                    (download as f64 / elapsed, upload as f64 / elapsed)
                }
                _ => (0.0, 0.0),
            };

            // The first sample only spans the time since the initial refresh, so
            // skip it instead of plotting a meaningless spike.
            if elapsed.is_some() {
                download_history.push(download_speed.round() as u64, width);
                upload_history.push(upload_speed.round() as u64, width);
            }

            let io = Io {
                disks: disks::Disks { disks },
                network: network::Network {
                    total_download,
                    total_upload,
                    download_speed,
                    upload_speed,
                    history_rx: download_history.clone(),
                    history_tx: upload_history.clone(),
                },
            };

            if tx.send(io).is_err() {
                break;
            }

            std::thread::sleep(std::time::Duration::from_secs(io::REFRESH_IN_SECS));
        }
    });

    rx
}
