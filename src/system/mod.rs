pub mod cpu;
pub mod history;
pub mod memory;
pub mod process;
pub mod workers;

#[derive(Debug, Default)]
pub struct SystemStats {
    pub memory_used: memory::Memory,
    pub memory_available: memory::Memory,
    pub memory_free: memory::Memory,
    pub memory_total: memory::Memory,
    pub cpu: cpu::Cpu,
}

pub struct SystemInfo {
    pub host_name: String,
    pub os_name: String,
    pub user_name: String,
    pub size_formatter: Box<dyn Fn(u64) -> String>,
}
