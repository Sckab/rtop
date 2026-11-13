mod process;
mod temperature;

pub use process::*;
pub use temperature::*;

#[derive(Default, Clone, PartialEq, Eq)]
pub enum SortDirection {
    #[default]
    Ascending,
    Descending,
}
