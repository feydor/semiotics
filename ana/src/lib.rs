pub mod gram {
    use std::path::Path;
    use std::fs::File;
    use std::io::{self, BufRead};
    use itertools::Itertools;
    
    #[derive(Debug)]
    pub struct Dict {
        // stored in alphabetical order
        entries: Vec<(String, String)>
    }

    impl Dict {
        pub fn new() -> Dict {
            Dict {
                entries: Vec::new(),
            }
        }

        // stores a copy of word in vec
        pub fn load(&mut self, word: &str) {
            let entry = (
                word.to_string(),
                word.chars().sorted().rev().collect(),
            );
            self.entries.push(entry);
        }

        pub fn anagrams(&self, input: &str) -> Vec<&str> {
            let mut res: Vec<&str> = vec![];
            let sorted_input = input.chars().sorted().rev().collect::<String>();
            for (entry, ordered) in self.entries.iter() {
                if  sorted_input == *ordered && input != entry {
                    res.push(entry);
                }
            }
            res
        }

        // returns an iterator over the lines of the filename
        fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path> {
            let file = File::open(filename)?;
            Ok(io::BufReader::new(file).lines())
        }

        pub fn load_from_file(&mut self, filepath: &str) {
            if let Ok(lines) = Dict::read_lines(filepath) {
                for line in lines {
                    if let Ok(word) = line {
                        self.load(&word);
                    }
                }
            }
        }
    }

    // if not alphabetical, returns 0
    fn ord(ch: char) -> usize {
        let alphabet = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
        'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
        match alphabet.iter().position(|&x| x == ch) {
            Some(i) => i,
            _ => 0
        }
    }

    // ascii 4ever
    pub fn str_ord(s: &str) -> usize {
        return s.chars().map(|ch| ord(ch)).sum()
    }

    // returns a new string word without the first occurrence of letter
    pub fn without_letter(word: &str, letter: char) -> Option<String> {
        match word.split_once(letter) {
            None => None,
            Some((first, second)) => Some(first.to_string() + second),
        }
    }

    // returns a new string word without the occurrences of each letter in letters
    // if no letters are found, returns the original string
    pub fn without_letters(word: &str, letters: &[char], start: usize) -> String {
        if start == letters.len() {
            return word.to_owned();
        }

        match without_letter(word, letters[start]) {
            Some(remaining) => without_letters(&remaining, letters, start + 1),
            None => without_letters(word, letters, start + 1),
        }
    }

    // same as without_letters but with the letters_used parameter populated
    fn without_letters_returned(word: &str, letters: &[char], letters_used: &mut Vec<char>, start: usize) -> String {
        if start == letters.len() {
            return word.to_owned();
        }

        match without_letter(word, letters[start]) {
            Some(rem) => {
                letters_used.push(letters[start]);
                without_letters_returned(&rem, letters, letters_used, start+1)
            },
            None => without_letters_returned(word, letters, letters_used, start+1)
        }
    }

    // returns a new sentence with the first occurene of each of the letters is removed
    pub fn remove_letters_from_sentence(sentence: &[&str], letters: &[char]) -> Vec<String> {
        let mut remaining_sentence = Vec::new();
        let mut letters_vec: Vec<char> = letters.to_vec();
        for word in sentence {
            let mut letters_used = Vec::new();
            let result = without_letters_returned(&word, &letters_vec, &mut letters_used, 0);
            if result.len() < word.len() {
                for used in letters_used {
                    match letters_vec.iter().position(|&a| a == used) {
                        Some(idx) => {letters_vec.remove(idx);},
                        _ => {},
                    }
                }
            }

            if result.len() > 0 {
                remaining_sentence.push(result);
            }
        }

        remaining_sentence
    }

    // TODO: two heap allocations just for to check for anagrams lol
    pub fn are_anagrams(a: &str, b: &str) -> bool {
        a.chars().sorted().rev().collect::<String>() == b.chars().sorted().rev().collect::<String>()
    }

    pub fn single_word_anagrams<'a>(words: &[&str], dict: &'a [&str]) -> Vec<&'a str> {
        let mut anagrams = Vec::new();
        for w in words {
            for &d in dict {
                if are_anagrams(w, d) {
                    anagrams.push(d);
                }
            }
        }
        
        anagrams
    }
}

#[cfg(test)]
mod tests {
    use crate::gram::*;
    #[test]
    fn str_ord_works() {
        assert_eq!(str_ord("abc"), 3);
        assert_eq!(str_ord("lewis"), 63);
        assert_eq!(str_ord("wiles"), 63);
    }

    #[test]
    fn single_word_anagrams_works() {
        let words = ["lewis"];
        let dict = ["apple", "box", "cat", "wiles", "zoo"];
        let res = single_word_anagrams(&words, &dict);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], "wiles");
    }

    #[test]
    fn without_letter_works() {
        assert_eq!(without_letter("word", 'w'), Some("ord".to_owned()));
        assert_eq!(without_letter("word", 'o'), Some("wrd".to_owned()));
        assert_eq!(without_letter("word", 'd'), Some("wor".to_owned()));
        assert_eq!(without_letter("word", 'z'), None);
    }

    #[test]
    fn without_letters_works() {
        let word = "haman";
        let letters = ['h', 'a', 'm'];
        assert_eq!(without_letters(&word, &letters, 0), "an".to_owned());
        assert_eq!(without_letters(&word, &['z'], 0), word.to_owned());
        assert_eq!(without_letters(&word, &[], 0), word.to_owned());
    }

    #[test]
    fn remove_letters_from_sentence_works() {
        let sentence = ["haman", "is", "cool"];
        let letters = ['h', 'i', 'c'];
        let res = vec!["aman", "s", "ool"];
        assert_eq!(remove_letters_from_sentence(&sentence, &letters), res);

        let letters = ['h', 'a', 'm', 'a', 'n'];
        let res = vec!["is", "cool"];
        assert_eq!(remove_letters_from_sentence(&sentence, &letters), res);

        let letters = ['a', 'h', 'i', 'l', 'm', 's'];
        let res = vec!["an", "coo"];
        assert_eq!(remove_letters_from_sentence(&sentence, &letters), res);

        let sentence = ["aaa"];
        let letters = ['a', 'a', 'a', 'a'];
        assert_eq!(remove_letters_from_sentence(&sentence, &letters).len(), 0);

        assert_eq!(remove_letters_from_sentence(&["tab", "target"], &['t', 't']), vec!["ab", "arget"]);
    }
}
