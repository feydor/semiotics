pub mod ana {
    fn ord(ch: char) -> usize {
        let alphabet = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
        'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
        alphabet.iter().position(|&x| x == ch).unwrap()
    }

    // ascii 4ever
    pub fn str_ord(s: &str) -> usize {
        return s.chars().map(|ch| ord(ch)).sum()
    }

    pub fn single_word_anagrams(words: &Vec<&str>, dict: &Vec<&str>) -> Vec<String> {
        let mut anagrams = Vec::new();
        let dict_ords: Vec<usize> = dict.iter().map(|&s| str_ord(s)).collect();
        for n in words.iter().map(|&s| str_ord(s)) {
            for (i, &ord) in dict_ords.iter().enumerate() {
                if ord == n {
                    anagrams.push(dict[i].to_string());
                }
            }
        }
        
        return anagrams;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn str_ord_works() {
        assert_eq!(crate::ana::str_ord("abc"), 3);
        assert_eq!(crate::ana::str_ord("lewis"), 63);
        assert_eq!(crate::ana::str_ord("wiles"), 63);
    }

    #[test]
    fn single_word_anagrams_works() {
        let words = ["lewis"].to_vec();
        let dict = ["apple", "box", "cat", "wiles", "zoo"].to_vec();
        let res = crate::ana::single_word_anagrams(&words, &dict);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], "wiles");
    }
}
