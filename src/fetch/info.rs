use human_units::si::{Frequency, Prefix};
use ratatui::{style::Stylize, text::Span, widgets::Row};
use time::Duration;

use crate::{
    fetch::get_maximum_frequency,
    theme::{Theme, color::Color},
};

pub struct Info {
    pub host_name: String,
    pub user_name: String,
    pub os: String,
    pub kernel: String,
    pub uptime: String,
    pub shell: String,
    pub cpu: String,
    pub frequency: String,
    pub gpus: Vec<retch_sysinfo::gpu::GpuInfo>,
    pub used_memory: u64,
    pub total_memory: u64,
    pub used_swap: u64,
    pub total_swap: u64,
    pub displays: Vec<String>,
    pub disks: Vec<String>,
    pub wm: String,
    pub terminal: String,
}

impl Info {
    pub fn new<F>(formatter: F) -> anyhow::Result<Self>
    where
        F: Fn(u64) -> String,
    {
        let uptime = Duration::seconds(sysinfo::System::uptime()? as i64);

        let hours = uptime.whole_hours();
        let minutes = uptime.whole_minutes() % 60;
        let seconds = uptime.whole_seconds() % 60;

        let uptime = format!("{hours}h {minutes}m {seconds}s");

        let system = sysinfo::System::new_all()?;
        let disks = sysinfo::Disks::new_with_refreshed_list()?;

        let mut frequency = 0_u64;

        for cpu in system.cpus() {
            frequency += cpu.frequency() * 1_000_000;
        }

        frequency /= system.cpus().len() as u64;

        Ok(Self {
            host_name: whoami::hostname().unwrap_or_else(|_| "Unknown".to_string()),
            user_name: whoami::username().unwrap_or_else(|_| "Unknown".to_string()),
            os: format!(
                "{} {} {}",
                sysinfo::System::name()?,
                sysinfo::System::os_version()?,
                sysinfo::System::cpu_arch()
            ),
            kernel: sysinfo::System::kernel_long_version()?,
            uptime,
            shell: whattheshell::Shell::infer()?.to_string(),
            cpu: format!(
                "{} @ {}",
                system.cpus()[0].brand().to_string(),
                Frequency::with_si_prefix(frequency, Prefix::None)
                    .format_si()
                    .to_string()
            ),
            frequency: if let Some(f) = get_maximum_frequency::get_maximum_frequency()? {
                Frequency::with_si_prefix(f, Prefix::None)
                    .format_si()
                    .to_string()
            } else {
                "".to_string()
            },
            gpus: retch_sysinfo::gpu::detect_gpus(),
            used_memory: system.used_memory(),
            total_memory: system.total_memory(),
            used_swap: system.used_swap(),
            total_swap: system.total_swap(),
            displays: retch_sysinfo::display::detect_displays(),
            disks: disks
                .iter()
                .map(|d| {
                    let used = d.total_space() - d.available_space();

                    format!(
                        "{} {} / {} ({}%) [{} | {}]",
                        if d.mount_point() == "/" {
                            "/".into()
                        } else {
                            d.mount_point()
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                        },
                        formatter(used),
                        formatter(d.total_space()),
                        used * 100 / d.total_space(),
                        d.kind().to_string(),
                        d.file_system().display(),
                    )
                })
                .collect(),
            wm: retch_sysinfo::wm::detect_wm().unwrap_or("Unknown".to_string()),
            terminal: format!(
                "{} {}",
                detect_terminal::detect()
                    .term
                    .unwrap_or("Unknwon".to_string()),
                detect_terminal::detect().version.unwrap_or_default()
            ),
        })
    }

    pub fn get_rows<F>(&self, theme: &Theme, formatter: F) -> Vec<Row<'_>>
    where
        F: Fn(u64) -> String,
    {
        let mut rows = vec![
            get_row(theme.primary, "OS", &self.os),
            get_row(theme.primary, "Kernel", &self.kernel),
            get_row(theme.primary, "Uptime", &self.uptime),
            get_row(theme.primary, "Shell", &self.shell),
        ];

        for display in &self.displays {
            rows.push(get_row(theme.primary, "Display", display));
        }

        rows.append(&mut vec![
            get_row(theme.primary, "CPU", &self.cpu),
            get_row(theme.primary, "Frequency", &self.frequency),
        ]);

        for gpu in &self.gpus {
            let frequency = gpu
                .clock_mhz
                .map(|mhz| {
                    format!(
                        " @ {}",
                        Frequency::with_si_prefix(mhz as u64 * 1_000_000, Prefix::None).format_si()
                    )
                })
                .unwrap_or_default();

            let specs = match (gpu.cores, gpu.vram_bytes) {
                (None, None) => String::new(),
                (cores, vram) => format!(
                    " ({})",
                    [
                        cores.map(|c| format!("{c} cores")),
                        vram.map(|v| formatter(v)),
                    ]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>()
                    .join(", ")
                ),
            };

            let gpu_type = gpu
                .gpu_type
                .clone()
                .map(|kind| format!(" [{kind}]"))
                .unwrap_or_default();

            rows.push(get_row(
                theme.primary,
                "GPU",
                format!("{}{}{}{}", gpu.name, frequency, specs, gpu_type),
            ));
        }

        rows.append(&mut vec![
            get_row(
                theme.primary,
                "Memory",
                format_memory(self.used_memory, self.total_memory, &formatter),
            ),
            get_row(
                theme.primary,
                "Swap",
                format_memory(self.used_swap, self.total_swap, formatter),
            ),
        ]);

        for disk in &self.disks {
            rows.push(get_row(theme.primary, "Disk", disk.to_owned()));
        }

        rows.append(&mut vec![
            get_row(theme.primary, "Terminal", &self.terminal),
            get_row(theme.primary, "WM", &self.wm),
        ]);

        rows
    }
}

fn get_row<'a, S>(color: Color, name: &'a str, value: S) -> Row<'a>
where
    S: AsRef<str> + 'a,
{
    Row::new([name.fg(color).bold(), Span::raw(value.as_ref().to_owned())])
}

fn format_memory<F>(used_memory: u64, total_memory: u64, formatter: F) -> String
where
    F: Fn(u64) -> String,
{
    format!(
        "{} / {} ({}%)",
        formatter(used_memory),
        formatter(total_memory),
        (used_memory * 100) / total_memory
    )
}
