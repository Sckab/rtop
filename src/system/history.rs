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
        if self.values.len() >= width && width > 0 {
            self.values.pop_front();
        }

        self.values.push_back(value);
    }
}
