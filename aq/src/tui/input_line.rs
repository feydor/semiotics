//! input_line.rs - cursor and cli line abstraction
#[derive(Default, Clone)]
pub struct InputLine {
    cursor: usize, // range: [0 (first char), line.len() (after last char)]
    line: String,
}

impl InputLine {
    pub fn from_string(line: String) -> Self {
        InputLine {
            cursor: line.len(),
            line,
        }
    }

    pub fn into_string(self) -> String {
        self.line
    }

    pub fn as_str(&self) -> &str {
        &self.line
    }

    pub fn len(&self) -> usize {
        self.line.len()
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

     pub fn insert(&mut self, ch: char) {
        // end of line insert (push)
        if self.cursor > self.line.len() {
            self.line.push(ch);
            self.cursor = self.line.len() + 1;
        } else {
            // inbetween line insert
            self.line = self
                .line
                .chars()
                .take(self.cursor) // chars up to cursor
                .chain(std::iter::once(ch)) // plus new character
                .chain(self.line.chars().skip(self.cursor)) // plus chars after cursor
                .collect();
            self.cursor += 1;
        }
    }

    pub fn cursor_left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    // increment cursor up to line.len (one char after end)
    pub fn cursor_right(&mut self) {
        self.cursor = self.line.len().min(self.cursor + 1); // smaller of two
    }

    // delete char before cursor
    pub fn backspace(&mut self) {
        if self.cursor == 0 {
            return;
        }

        if self.cursor > self.line.len() {
            self.line.pop();
        } else {
            // range: [0, cursor-1)..[cursor, line.len()]
            self.line = self
                .line
                .chars()
                .take(self.cursor - 1)
                .chain(self.line.chars().skip(self.cursor))
                .collect()
        }
        self.cursor -= 1;
    }

    // delete char on cursor
    pub fn delete_key(&mut self) {
        // cursor after last char of line; do nothing
        if self.cursor == self.line.len() {
            return;
        }

        // cursor is off the rails; empty line
        if self.cursor > self.line.len() {
            self.line.pop();
        } else {
            // delete the char at cursor
            // range: [0, cursor)..[cursor+1,line.len()]
            self.line = self
            .line
            .chars()
            .take(self.cursor)
            .chain(self.line.chars().skip(self.cursor + 1))
            .collect();
        }
    }
}
