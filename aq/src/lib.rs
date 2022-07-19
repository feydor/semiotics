//! aq - gematric and decimation functions for A. Barrow's Anglossic Qabbala (AQ)

// alphanumeric ciphers go here
const AQ: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

// full digital-reduction of any query string using August Barrow's method of Anglossic Qabbala
// EX: nummificate("aok") -> [54, 9]
pub fn nummificate(query: &str) -> Vec<i32> {
    let mut res = Vec::<i32>::new();
    let mut n = gematria(&query, AQ);
    res.push(n);

    while !is_single_digit(&n) {
        n = decimate(&n);
        res.push(n);
    }
    return res;
}

// English => AlphaNumerical => Numerical (via AQ or really any cipher mapping 0-9A-Z)
// Note: query must be uppercase
// Note: query can be non-alphanumerical input (it will be ignored in the calculation)
// EX: gematria("aok", "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ") -> 54
fn gematria(query: &str, cipher: &str) -> i32 {
    query.chars()
         .map(|ch| cipher.find(ch).unwrap_or_default() as i32)
         .sum()
}

// decimation; digital reduction; plexing; modulo-summation
// EX: 140 => 5, 999 => 27
fn decimate(n: &i32) -> i32 {
    match is_single_digit(&n) {
        true => n.abs(),
        false => n.abs() % 10 + decimate(&(n / 10)),
    }
}

fn is_single_digit(n: &i32) -> bool {
    n.abs() < 10
}

// every three chars in query generates an RGB hex values
// ignores chars at end, where i % 3 == 0
// src: https://northanger.livejournal.com/272673.html
pub fn hex_triplets(query: &str) -> Vec<(u8, u8, u8)> {
    let mut hex_trips = Vec::<(u8, u8, u8)>::new();
    let mut i = 0;
    let mut arr: [u8; 3] = [0, 0, 0];
    for ch in query.chars() {
        arr[i] = nummificate(&ch.to_string().to_uppercase())[0] as u8;
        i += 1;
        if i % 3 == 0 {
            hex_trips.push((arr[0], arr[1], arr[2]));
            i = 0;
        }
    }
    hex_trips
}

pub fn hex_trinome(query: &str) -> (u8, u8, u8) {
    let n = nummificate(&query.to_uppercase())[0];
    ((n & 0x00FF0000 >> 16) as u8,
    (n & 0x0000FF00 >> 8) as u8,
    n as u8)
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn nummificate_works() {
    	assert_eq!(nummificate(&"AOK"), vec![54, 9]);
    }

    #[test]
    fn gematria_works() {
    	assert_eq!(gematria(&"ZERO", AQ), 100);
        assert_eq!(gematria(&"zero", AQ), 0); // ignore lowercase
        assert_eq!(gematria(&"😅", AQ), 0); // ignore non-ascii characters
    }

    #[test]
    fn decimation_works() {
    	assert_eq!(decimate(&10), 1);
    }
}
