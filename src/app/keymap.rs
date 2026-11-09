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

pub fn default_keys() -> Vec<Key> {
    vec![
        Key::new("h", "?", "Shows this help panel"),
        Key::new("q", "Esc", "Quit the application (or this panel)"),
        Key::new("k", "Up", "Select previous process"),
        Key::new("j", "Down", "Select next process"),
        Key::new("g", "Left", "Select first process"),
        Key::new("G", "Right", "Select last process"),
        Key::new("K", "", "Kill selected process"),
        Key::new("/", "", "Filter processes"),
    ]
}
