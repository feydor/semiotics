//! history.rs - query storage with movable index
use std::collections::VecDeque;
use std::collections::HashMap;

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

    // returns all entries with matching aq numbers
    // removes duplicates before returning
    // returns:
    // hashmap {
    //      249: ["outsideness", "xenosytem"],
    //        n: ["a word", "another", "matches here", ...]
    // }
    pub fn matches(&self) -> Option<HashMap<i32, Vec<String>>> {
        let mut matches: HashMap<i32, Vec<String>> = HashMap::new();

        // populate hashmap, one key : many values
        for (aq, query) in &self.history {
            matches.entry(*aq)
                .or_default()
                .push(query.to_string());
        }

        // remove duplicates
        for (_, vec) in matches.iter_mut() {
            vec.sort();
            vec.dedup();
        }

        // delete entries without matches
        matches = matches.into_iter()
                         .filter(|(_, vec)| vec.len() > 1)
                         .collect();

        if matches.is_empty() {
            return None;
        }

        return Some(matches);

/*
        let mut res = self.history
            .clone()
            .into_iter()
            .filter(|&(i, _)| i == n)   // tuples with matching index
            .map(|(_,s)| s)             // strip query
            .filter(|s| *s != *query)   // strip duplicates
            .collect::<Vec<String>>();
        res.push(query.to_string());   // adds query to returned matches
        if res.len() < 2 {
            None
        } else {
            Some(res)
        }
        */
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
