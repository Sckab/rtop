use crate::system::{sorting::SortDirection, temperature::Temperature};

#[derive(Default, Clone, PartialEq, Eq)]
pub enum TemperatureSortType {
    #[default]
    Sensor,
    Temperature,
    None,
}

#[derive(Default, Clone)]
pub struct TemperatureSort {
    sort_type: TemperatureSortType,
    sort_direction: SortDirection,
}

impl TemperatureSort {
    pub fn new(sort_type: TemperatureSortType, shift: bool) -> Self {
        Self {
            sort_type,
            sort_direction: if !shift {
                SortDirection::Ascending
            } else {
                SortDirection::Descending
            },
        }
    }

    pub fn sort_temperatures(&self, temperatures: &mut [Temperature]) {
        let compare = |a: &Temperature, b: &Temperature| {
            let ordering = match self.sort_type {
                TemperatureSortType::Sensor => a.sensor.cmp(&b.sensor),
                TemperatureSortType::Temperature => match (a.temperature, b.temperature) {
                    (Some(a), Some(b)) => a.total_cmp(&b),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => std::cmp::Ordering::Equal,
                },
                TemperatureSortType::None => std::cmp::Ordering::Equal,
            };

            match self.sort_direction {
                SortDirection::Ascending => ordering,
                SortDirection::Descending => ordering.reverse(),
            }
        };

        if self.sort_type != TemperatureSortType::None {
            temperatures.sort_by(|a, b| compare(a, b));
        }
    }

    pub fn get_arrow(&self, target: TemperatureSortType) -> &str {
        if self.sort_type != target {
            return "";
        }

        match self.sort_direction {
            SortDirection::Ascending => " ▲",
            SortDirection::Descending => " ▼",
        }
    }
}
