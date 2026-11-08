#[derive(Debug, Default, Clone)]
pub struct Memory {
    pub value: u64,
    pub history: super::history::History<u64>,
}
