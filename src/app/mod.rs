use crossterm::event::{self, Event, KeyEventKind};
use ratatui::DefaultTerminal;
use std::io;

use crate::{
    app::state::State,
    system::{
        SystemInfo, SystemStats,
        io::{Io, disks::Disk},
        process::{KillRequest, KillResponse, Process, ProcessesState},
        sorting::Sortings,
        temperature::{Temperature, TemperatureState},
        workers,
    },
    theme::Theme,
    tui::NETWORK_PANEL_PERCENT,
};

mod input;
pub mod keymap;
pub mod state;

use state::{TableStates, Toast};

pub struct App {
    pub quit: bool,
    pub state: State,
    pub toast: Option<Toast>,
    pub system_info: SystemInfo,
    pub stats: SystemStats,
    pub tables: TableStates, // we still leave this, it could be useful in the future
    pub keys: keymap::Keys,
    pub processes_state: ProcessesState,
    pub temperature_state: TemperatureState,
    pub sortings: Sortings,
    pub io: Io,
    pub size_formatter: Box<dyn Fn(u64) -> String>,
    pub theme: Theme,
}

impl App {
    pub fn new() -> Self {
        let system_info = SystemInfo {
            host_name: whoami::hostname().unwrap_or_else(|_| "Unknown".to_string()),
            os_name: whoami::distro().unwrap_or_else(|_| "Unknown".to_string()),
            user_name: whoami::username().unwrap_or_else(|_| "Unknown".to_string()),
        };

        Self {
            quit: false,
            state: State::default(),
            toast: None,
            system_info,
            stats: SystemStats::default(),
            tables: TableStates::default(),
            keys: keymap::Keys::new(),
            processes_state: ProcessesState::default(),
            temperature_state: TemperatureState::default(),
            sortings: Sortings::default(),
            io: Io::default(),
            size_formatter: Box::new(humansize::make_format(humansize::DECIMAL)),
            theme: Theme::default(),
        }
    }

    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        stats_system: sysinfo::System,
        process_system: sysinfo::System,
        components: sysinfo::Components,
        disks: sysinfo::Disks,
        network: sysinfo::Networks,
        users: sysinfo::Users,
    ) -> io::Result<()> {
        let (tx_kill, rx_kill) = std::sync::mpsc::channel::<KillRequest>();

        let (tx_cpu_ram_width, rx_cpu_ram_width) = std::sync::mpsc::channel::<usize>();
        let (tx_network_width, rx_network_width) = std::sync::mpsc::channel::<usize>();

        let terminal_width = terminal.size()?.width;
        let _ = tx_cpu_ram_width.send((terminal_width / 2) as usize);
        let _ =
            tx_network_width.send(terminal_width as usize * NETWORK_PANEL_PERCENT as usize / 100);

        let rx_stats = workers::stats::create_worker(rx_cpu_ram_width, stats_system);
        let (rx_processes, rx_msg_kill) =
            workers::processes::create_worker(rx_kill, process_system, users);
        let rx_temperatures = workers::temperatures::create_worker(components);
        let rx_io = workers::io::create_worker(rx_network_width, network, disks);

        let mut dirty = true;

        while !self.quit {
            let terminal_width = terminal.size()?.width;
            let _ = tx_cpu_ram_width.send((terminal_width / 2) as usize);
            let _ = tx_network_width
                .send(terminal_width as usize * NETWORK_PANEL_PERCENT as usize / 100);

            if self.toast.as_ref().is_some_and(Toast::is_expired) {
                self.toast = None;
                dirty = true;
            }

            if let Some(pid) = self.processes_state.pending_kill.take() {
                let _ = tx_kill.send(pid);
            }

            while let Ok(stats) = rx_stats.try_recv() {
                self.stats = stats;
                dirty = true;
            }

            while let Ok(response) = rx_msg_kill.try_recv() {
                let message = match response {
                    // This shouldn't happen, but the process may have already exited
                    // by the time the user confirms the kill request.
                    KillResponse::NotFound(pid) => {
                        format!("{pid} not found.")
                    }
                    KillResponse::KillFailed(pid) => {
                        format!("Failed to kill {pid}")
                    }
                };

                self.toast = Some(Toast::new(message));
                dirty = true;
            }

            while let Ok(mut processes) = rx_processes.try_recv() {
                let selected = self.update_processes(&mut processes);

                self.processes_state.processes = processes;
                self.tables.process_table.select(selected);

                dirty = true;
            }

            while let Ok(mut temperatures) = rx_temperatures.try_recv() {
                let selected = self.update_temperatures(&mut temperatures);

                self.temperature_state.temperatures = temperatures;
                self.tables.temperature_table.select(selected);

                dirty = true;
            }

            while let Ok(mut io) = rx_io.try_recv() {
                let selected = self.update_disks(&mut io.disks.disks);

                self.io = io;
                self.tables.disks_table.select(selected);

                dirty = true;
            }

            if self.handle_events()? {
                dirty = true;
            }

            if dirty {
                terminal.draw(|frame| self.render(frame.area(), frame.buffer_mut()))?;
                dirty = false;
            }
        }

        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<bool> {
        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event);
                    Ok(true)
                }
                Event::Resize(_, _) => Ok(true),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    fn resort_temperatures(&mut self) {
        let selected_sensor = self
            .tables
            .temperature_table
            .selected()
            .and_then(|index| self.temperature_state.temperatures.get(index))
            .map(|temperature| temperature.sensor.clone());

        self.sortings
            .temperatures
            .sort_temperatures(&mut self.temperature_state.temperatures);

        let selected = selected_sensor
            .and_then(|sensor| {
                self.temperature_state
                    .temperatures
                    .iter()
                    .position(|temperature| temperature.sensor == sensor)
            })
            .or_else(|| (!self.temperature_state.temperatures.is_empty()).then_some(0));

        self.tables.temperature_table.select(selected);
    }

    fn update_temperatures(&self, temepratures: &mut [Temperature]) -> Option<usize> {
        self.sortings.temperatures.sort_temperatures(temepratures);

        let selected_temp = self
            .tables
            .temperature_table
            .selected()
            .and_then(|index| self.temperature_state.temperatures.get(index))
            .map(|temperature| temperature.sensor.as_str());

        selected_temp
            .and_then(|temperature| {
                temepratures
                    .iter()
                    .position(|temp| temp.sensor == temperature)
            })
            .or_else(|| {
                if temepratures.is_empty() {
                    None
                } else {
                    Some(
                        self.tables
                            .temperature_table
                            .selected()
                            .unwrap_or(0)
                            .min(temepratures.len() - 1),
                    )
                }
            })
    }

    fn resort_disks(&mut self) {
        let selected_mount = self
            .tables
            .disks_table
            .selected()
            .and_then(|index| self.io.disks.disks.get(index))
            .map(|disk| disk.mount.clone());

        self.sortings.disks.sort_disks(&mut self.io.disks.disks);

        let selected = selected_mount
            .and_then(|mount| {
                self.io
                    .disks
                    .disks
                    .iter()
                    .position(|disk| disk.mount == mount)
            })
            .or_else(|| {
                if self.io.disks.disks.is_empty() {
                    None
                } else {
                    Some(
                        self.tables
                            .disks_table
                            .selected()
                            .unwrap_or(0)
                            .min(self.io.disks.disks.len() - 1),
                    )
                }
            });

        self.tables.disks_table.select(selected);
    }

    fn update_disks(&self, disks: &mut [Disk]) -> Option<usize> {
        self.sortings.disks.sort_disks(disks);

        let selected_disk = self
            .tables
            .disks_table
            .selected()
            .and_then(|index| self.io.disks.disks.get(index))
            .and_then(|disk| disk.mount.to_str());

        selected_disk
            .and_then(|mount| {
                disks
                    .iter()
                    .position(|disk| disk.mount.to_str() == Some(mount))
            })
            .or_else(|| {
                if disks.is_empty() {
                    None
                } else {
                    Some(
                        self.tables
                            .disks_table
                            .selected()
                            .unwrap_or(0)
                            .min(disks.len() - 1),
                    )
                }
            })
    }

    fn update_processes(&self, processes: &mut [Process]) -> Option<usize> {
        for process in processes.iter_mut() {
            crate::tui::prepare_display(process, &self.size_formatter);
        }

        let selected_pid = self
            .tables
            .process_table
            .selected()
            .and_then(|index| self.processes_state.processes.get(index))
            .map(|process| process.pid);

        selected_pid
            .and_then(|pid| processes.iter().position(|process| process.pid == pid))
            .or_else(|| {
                if processes.is_empty() {
                    None
                } else {
                    Some(
                        self.tables
                            .process_table
                            .selected()
                            .unwrap_or(0)
                            .min(processes.len() - 1),
                    )
                }
            })
    }
}
