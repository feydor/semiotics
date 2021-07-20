//! aq - gematric and decimation functions for A. Barrow's Anglossic Qabbala (AQ)
use lazy_static::lazy_static;
const ALPHANUM: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
lazy_static! {
	// alphanumeric ciphers go here
    static ref AQ: Vec<i32> = (0..36).collect();
}

// full digital-reduction of any query string using August Barrow's method of Anglossic Qabbala
// EX: nummificate("aok") -> [54, 9]
pub fn nummificate(query: &str) -> Vec<i32> {
    let mut res = Vec::<i32>::new();
    let mut n = aq(&query.to_string());
    res.push(n);

    while !is_single_digit(&n) {
        n = decimate(&n);
        res.push(n);
    }
    return res;
}

// English => AlphaNumerical => Numerical (via AQ or really any cipher spanning 0-9A-Z)
// Note: query must be uppercase
// Note: query can be non-alphanumerical input (it will be ignored in the calculation)
// EX: aq("aok") -> 54
fn aq(query: &String) -> i32 {
    if query.is_empty() {
        return 0;
    }
    let mut chars: Vec<_> = query.split("").collect();
    chars.remove(0);
    chars.pop();

    // index the query char-wise from last to first into ALPHANUM,
    // giving the index of the numerical value in AQ; sum it
    let curr = chars.pop().expect("query is empty!");
    let i: usize;
    match ALPHANUM.find(curr) {
        None => return 0 + aq(&chars.join("")),
        Some(index) => i = index,
    }
    return AQ[i] + aq(&chars.join(""));
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

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn nummificate_works() {
    	assert_eq!(nummificate(&"AOK".to_string()), vec![54, 9]);
    }

    #[test]
    fn aq_works() {
    	assert_eq!(aq(&"ZERO".to_string()), 100);
    }

    #[test]
    fn decimation_works() {
    	assert_eq!(decimate(&10), 1);
    }
}
