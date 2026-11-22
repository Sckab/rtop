use crate::system::{io::disks::Disk, sorting::SortDirection};

#[derive(Default, Clone, PartialEq, Eq)]
pub enum DisksSortType {
    #[default]
    Name,
    MountPoint,
    Used,
    Available,
    Total,
    Read,
    Write,
    None,
}

#[derive(Default, Clone)]
pub struct DisksSort {
    sort_type: DisksSortType,
    sort_direction: SortDirection,
}

impl DisksSort {
    pub fn new(sort_type: DisksSortType, shift: bool) -> Self {
        Self {
            sort_type,
            sort_direction: if !shift {
                SortDirection::Ascending
            } else {
                SortDirection::Descending
            },
        }
    }

    pub fn sort_disks(&self, disks: &mut [Disk]) {
        let compare = |a: &Disk, b: &Disk| {
            let ordering = match self.sort_type {
                DisksSortType::Name => a.name.cmp(&b.name),
                DisksSortType::MountPoint => a.mount.cmp(&b.mount),
                DisksSortType::Used => (a.total - a.available).cmp(&(b.total - b.available)),
                DisksSortType::Available => a.available.cmp(&b.available),
                DisksSortType::Total => a.total.cmp(&b.total),
                DisksSortType::Read => a.read.cmp(&b.read),
                DisksSortType::Write => a.write.cmp(&b.write),
                DisksSortType::None => std::cmp::Ordering::Equal,
            };

            match self.sort_direction {
                SortDirection::Ascending => ordering,
                SortDirection::Descending => ordering.reverse(),
            }
        };

        if self.sort_type != DisksSortType::None {
            disks.sort_by(|a, b| compare(a, b));
        }
    }

    pub fn get_arrow(&self, target: DisksSortType) -> &str {
        if self.sort_type != target {
            return "";
        }

        match self.sort_direction {
            SortDirection::Ascending => " ▲",
            SortDirection::Descending => " ▼",
        }
    }
}
