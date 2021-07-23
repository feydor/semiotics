//! history.rs - query storage with movable index
use std::collections::VecDeque;

const MAXHISTORY: usize = 100;

#[derive(Debug)]
pub struct History {
    history: VecDeque<(i32, String)>,
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
    pub fn current_query(&self) -> Option<&str> {
        self.history.get(self.index).map(|entry| entry.1.as_str())
    }

    pub fn current_number(&self) -> Option<i32> {
        self.history.get(self.index).map(|entry| entry.0)
    }

    pub fn save(&mut self, entry: (i32, String)) {
        if self.history.len() == self.max {
            self.history.pop_front();
        }
        self.history.push_back(entry);
        self.index = self.history.len();
    }

    // returns entries with a gematric number of n
    pub fn matches(&self, n: i32) -> Option<Vec<String>> {
        Some(self.history
            .clone()
            .into_iter()
            .filter(|&(i, _)| i == n)
            .map(|(_,s)| s)
            .collect())
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
