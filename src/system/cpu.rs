#[derive(Debug, Clone)]
pub struct CpuStats {
    pub number: usize,
    pub usage: f32,
}

#[derive(Debug, Default, Clone)]
pub struct Cpu {
    pub global_usage: f32,
    pub cpus: Vec<CpuStats>,
    pub history: super::history::History<f64>,
    pub frequency: u64,
    pub brand: String,
}
