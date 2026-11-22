#[derive(Clone)]
pub struct Key {
    pub primary_key: String,
    pub secondary_key: String,
    pub description: String,
}

impl Key {
    pub fn new(primary_key: &str, secondary_key: &str, description: &str) -> Self {
        Self {
            primary_key: primary_key.to_string(),
            secondary_key: secondary_key.to_string(),
            description: description.to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Keys {
    pub general: Vec<Key>,
    pub sorting_processes: Vec<Key>,
    pub sorting_temperatures: Vec<Key>,
    pub sorting_disks: Vec<Key>,
}

impl Keys {
    pub fn new() -> Self {
        let general = vec![
            Key::new("h", "?", "Shows this help panel"),
            Key::new("q", "Esc", "Quit the application"),
            Key::new("k", "Up", "Select previous row"),
            Key::new("j", "Down", "Select next row"),
            Key::new("g", "Left", "Select first row"),
            Key::new("G", "Right", "Select last row"),
            Key::new("K", "", "Move up"),
            Key::new("J", "", "Move down"),
            Key::new("H", "", "Move left"),
            Key::new("L", "", "Move right"),
            Key::new("Alt + D", "", "Kill selected process (Processes panel)"),
            Key::new("/", "", "Filter processes"),
            Key::new("Esc", "", "Cancel the filter input"),
            Key::new("Enter", "", "Confirm the filter"),
            Key::new("Alt + r", "", "Reset sorting"),
        ];

        let sorting_processes = vec![
            Key::new("Alt + p", "", "Sort by PID"),
            Key::new("Alt + n", "", "Sort by name"),
            Key::new("Alt + e", "", "Sort by executable"),
            Key::new("Alt + u", "", "Sort by user"),
            Key::new("Alt + t", "", "Sort by threads"),
            Key::new("Alt + c", "", "Sort by CPU usage"),
            Key::new("Alt + m", "", "Sort by memory usage"),
        ];

        let sorting_temperatures = vec![
            Key::new("Alt + s", "", "Sort by sensor"),
            Key::new("Alt + t", "", "Sort by temperature"),
        ];

        let sorting_disks = vec![
            Key::new("Alt + n", "", "Sort by name"),
            Key::new("Alt + m", "", "Sort by mount point"),
            Key::new("Alt + u", "", "Sort by used space"),
            Key::new("Alt + a", "", "Sort by available space"),
            Key::new("Alt + t", "", "Sort by total space"),
            Key::new("Alt + d", "", "Sort by read"),
            Key::new("Alt + w", "", "Sort by write"),
        ];

        Self {
            general,
            sorting_processes,
            sorting_temperatures,
            sorting_disks,
        }
    }
}
