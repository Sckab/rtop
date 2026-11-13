pub mod cpu;
pub mod history;
pub mod memory;
pub mod process;
pub mod sorting;
pub mod temperature;
pub mod workers;

#[derive(Debug, Default)]
pub struct SystemStats {
    pub memory_used: memory::Memory,
    #[cfg_attr(windows, allow(dead_code))]
    pub memory_available: memory::Memory,
    pub memory_free: memory::Memory,
    pub memory_total: memory::Memory,
    pub cpu: cpu::Cpu,
    pub load_average: LoadAverage,
}

pub struct SystemInfo {
    pub host_name: String,
    pub os_name: String,
    pub user_name: String,
    pub size_formatter: Box<dyn Fn(u64) -> String>,
}

#[derive(Default, Debug)]
pub struct LoadAverage {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
}

impl LoadAverage {
    pub fn display(&self) -> String {
        format!("{:.2}  {:.2}  {:.2}", self.one, self.five, self.fifteen)
    }
}
