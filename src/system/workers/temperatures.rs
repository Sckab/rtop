use std::sync::mpsc::Receiver;

use crate::system::temperature::Temperature;

pub fn create_worker(mut components: sysinfo::Components) -> Receiver<Vec<Temperature>> {
    let (tx, rx) = std::sync::mpsc::channel::<Vec<Temperature>>();

    std::thread::spawn(move || {
        loop {
            components.refresh(true);

            let temperatures: Vec<Temperature> = components
                .iter()
                .map(|component| Temperature {
                    sensor: component.label().to_string(),
                    temperature: component.temperature(),
                })
                .collect();

            if tx.send(temperatures).is_err() {
                break;
            }

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    rx
}
