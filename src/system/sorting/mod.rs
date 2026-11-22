mod disks;
mod process;
mod temperature;

pub use disks::*;
pub use process::*;
pub use temperature::*;

#[derive(Default, Clone)]
pub struct Sortings {
    pub processes: ProcessSort,
    pub temperatures: TemperatureSort,
    pub disks: DisksSort,
}

#[derive(Default, Clone, PartialEq, Eq)]
pub enum SortDirection {
    #[default]
    Ascending,
    Descending,
}
