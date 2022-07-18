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

pub fn trinomes(query: &str) -> String {
    let mut i = 0;
    let mut trinomes = Vec::<i32>::new();
    let mut output_str = "THE IRON LAW OF SIX\n".to_owned();
    for c in query.chars() {
        let trinome = nummificate(&c.to_string().to_uppercase())[0];
        trinomes.push(trinome);
        i += 1;
        if i % 3 == 0 {
            output_str += &hex_trinomes(&trinomes);
            trinomes.clear();
        }
    }
    output_str
}

// prints the hex trinome in color, using itself
fn hex_trinomes(trinomes: &Vec<i32>) -> String {
    if trinomes.len() < 3 {
        panic!("trinomes must be hex.");
    }

    let mut s = String::from(" ");
    for t in trinomes {
        s.push_str(format!("{:#04X} ", t).as_str());
    }
    s.push_str("\n");
    s
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
