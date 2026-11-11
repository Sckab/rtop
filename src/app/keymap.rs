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
    pub sorting: Vec<Key>,
}

impl Keys {
    pub fn new() -> Self {
        let general = vec![
            Key::new("h", "?", "Shows this help panel"),
            Key::new("q", "Esc", "Quit the application"),
            Key::new("k", "Up", "Select previous process"),
            Key::new("j", "Down", "Select next process"),
            Key::new("g", "Left", "Select first process"),
            Key::new("G", "Right", "Select last process"),
            Key::new("K", "", "Kill selected process"),
            Key::new("/", "", "Filter processes"),
            Key::new("Esc", "", "Cancel the filter input"),
            Key::new("Enter", "", "Confirm the filter"),
        ];

        let sorting = vec![
            Key::new("Alt + p", "", "Sort by PID"),
            Key::new("Alt + n", "", "Sort by name"),
            Key::new("Alt + e", "", "Sort by executable"),
            Key::new("Alt + u", "", "Sort by user"),
            Key::new("Alt + t", "", "Sort by threads"),
            Key::new("Alt + c", "", "Sort by CPU usage"),
            Key::new("Alt + m", "", "Sort by memory usage"),
            Key::new("Alt + r", "", "Reset process sorting"),
        ];

        Self { general, sorting }
    }

    pub fn len(&self) -> usize {
        self.general.len() + self.sorting.len()
    }
}
