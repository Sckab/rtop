use std::collections::VecDeque;

#[derive(Default, Debug, Clone)]
pub struct History<T> {
    pub values: VecDeque<T>,
    pub capacity: usize,
}

impl<T> History<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            values: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.values.len() >= self.capacity {
            self.values.pop_front();
        }

        self.values.push_back(value);
    }
}
