//! history.rs - query storage with movable index
use std::collections::VecDeque;

const MAXHISTORY: usize = 100;

#[derive(Debug)]
pub struct History {
    history: VecDeque<String>,
    index: usize,
    max: usize,
}

impl Default for History {
    fn default() -> Self {
        History {
            history: VecDeque::new(),
            index: 0,
            max: MAXHISTORY,
        }
    }
}

impl History {
    pub fn new(max: usize, history: VecDeque<String>) -> Self {
        History {
            index: history.len(),
            history,
            max,
        }
    }

    pub fn current(&self) -> Option<&str> {
        self.history.get(self.index).map(|s| s.as_str())
    }

    pub fn save(&mut self, item: String) {
        if self.history.len() == self.max {
            self.history.pop_front();
        }
        self.history.push_back(item);
        self.index = self.history.len();
    }

    // decrement history
    pub fn up(&mut self) {
        self.index = self.index.saturating_sub(1);
    }

    // increment history up to histroy.len()
    pub fn down(&mut self) {
        self.index = self.history.len().min(self.index + 1);
    }
}
