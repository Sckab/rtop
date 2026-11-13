use crate::system::{process::Process, sorting::SortDirection};

#[derive(Default, Clone, PartialEq, Eq)]
pub enum ProcessSortType {
    Pid,
    Name,
    Exe,
    User,
    Thread,
    Cpu,
    Memory,
    #[default]
    None,
}

#[derive(Default, Clone)]
pub struct ProcessSort {
    pub sort_type: ProcessSortType,
    pub sort_direction: SortDirection,
}

impl ProcessSort {
    pub fn new(sort_type: ProcessSortType, shift: bool) -> Self {
        Self {
            sort_type,
            sort_direction: if !shift {
                SortDirection::Ascending
            } else {
                SortDirection::Descending
            },
        }
    }

    pub fn sort_processes(&self, processes: &mut Vec<&Process>) {
        let compare = |a: &Process, b: &Process| {
            let ordering = match self.sort_type {
                ProcessSortType::Pid => a.pid.cmp(&b.pid),
                ProcessSortType::Name => a.name.cmp(&b.name),
                ProcessSortType::Exe => a.exe.cmp(&b.exe),
                ProcessSortType::User => a.user.cmp(&b.user),
                ProcessSortType::Thread => a.threads.cmp(&b.threads),
                ProcessSortType::Cpu => a.cpu_usage.total_cmp(&b.cpu_usage),
                ProcessSortType::Memory => a.memory.cmp(&b.memory),
                ProcessSortType::None => std::cmp::Ordering::Equal,
            };

            match self.sort_direction {
                SortDirection::Ascending => ordering,
                SortDirection::Descending => ordering.reverse(),
            }
        };

        if self.sort_type != ProcessSortType::None {
            processes.sort_by(|a, b| compare(a, b));
        }
    }

    pub fn get_arrow(&self, target: ProcessSortType) -> &str {
        if self.sort_type != target {
            return "";
        }

        match self.sort_direction {
            SortDirection::Ascending => " ▲",
            SortDirection::Descending => " ▼",
        }
    }
}
