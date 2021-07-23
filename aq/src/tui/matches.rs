//! matches.rs - matches storage
use std::collections::HashMap;

const MAX_MATCHES_PER_ENTRY: usize = 33;

#[derive(Debug)]
pub struct Matches {
    matches: HashMap<i32, Vec<String>>,
    max: usize,
}

impl Default for Matches {
    fn default() -> Self {
        Matches {
            matches: HashMap::new(),
            max: MAX_MATCHES_PER_ENTRY,
        }
    }
}

impl Matches {
    pub fn new(max: usize, matches: HashMap<i32, Vec<String>>) -> Self {
        Matches {
            matches,
            max,
        }
    }

    pub fn get_matches(&self, key: i32) -> Option<&Vec<String>> {
        if self.matches.contains_key(&key) {
            return self.matches.get(&key);
        }
        None
    }

    // adds the entries to the vector with param key
    pub fn save(&mut self, key: i32, matches: Vec<String>) {
        if !self.matches.contains_key(&key) {
            return;
        }
        if self.matches.contains_key(&key) && self.matches.get(&key).unwrap().len() == self.max {
            return;
        }

        self.matches
            .entry(key)
            .and_modify(|vec| vec.extend(matches.to_vec()))
            .or_insert(matches.to_vec());
    }
}
