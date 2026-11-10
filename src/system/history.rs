use std::collections::VecDeque;

#[derive(Default, Debug, Clone)]
pub struct History<T> {
    pub values: VecDeque<T>,
}

impl<T> History<T> {
    pub fn new() -> Self {
        Self {
            values: VecDeque::new(),
        }
    }

    pub fn push(&mut self, value: T, width: usize) {
        while self.values.len() >= width && !self.values.is_empty() {
            self.values.pop_front();
        }

        if width > 0 {
            self.values.push_back(value);
        }
    }
}
