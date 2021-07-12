/* aq.rs - gematric and digital reduction functions for Anglossic Qabbala (AQ) */
use std::env;
use lazy_static::lazy_static;
const PROJECT_NAME: &str = "aq";
const ALPHANUM: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

lazy_static! {
    static ref AQ: Vec<i32> = (0..36).collect();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} [alphanumeric string]", PROJECT_NAME);
        std::process::exit(1);
    }

    let query = String::from(&args[1]).chars().filter(|&c| c.is_alphanumeric() ).collect::<String>().to_uppercase();
    print!("query: {} == ", query);
    let output = nummificate(&query); // contains the complete digital reduction of query
    println!("{}", output[0]); // only output the first
}

// full digital-reduction of any query string using August Barrow's method of Anglossic Qabbala
// EX: nummificate("aok") -> [54, 9]
fn nummificate(query: &str) -> Vec<i32> {
    let mut res = Vec::<i32>::new();
    let mut n = aq(&String::from(query));
    res.push(n);

    while !is_single_digit(&n) {
        n = dreduce(&n);
        res.push(n);
    }
    return res;
}

// English => AlphaNumerical => Numerical (via AQ)
// NOTE: query can be non-alphanumerical input (it will be ignored in the calculation)
// EX: aq("aok") -> 54
fn aq(query: &String) -> i32 {
    if query.is_empty() { return 0 }
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
    return AQ[i] + aq(&chars.join(""))
}

// digital reduction; modulo-summation
// EX: 140 => 5, 999 => 27
fn dreduce(n: &i32) -> i32 {
    match is_single_digit(&n) {
        true => n.abs(),
        false => n.abs() % 10 + dreduce(&(n/10)),
    }
}

fn is_single_digit(n: &i32) -> bool {
    n.abs() < 10
}

