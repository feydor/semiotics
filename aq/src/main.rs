/* aq.rs - gematric and digital reduction functions for Anglossic Qabbala (AQ) */
use std::env;
use lazy_static::lazy_static;
const PROJECT_NAME: &str = "aq";
const ALPHANUM: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

lazy_static! {
    static ref AQ: Vec<i32> = (0..36).collect();
}

// const AQ: Vec<i32> = (0..36).collect();

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} [alphanumeric string]", PROJECT_NAME);
        std::process::exit(1);
    }
    let query = String::from(&args[1]).to_uppercase();
    print!("query: {} == ", &query.chars().filter(|&c| c.is_alphanumeric() ).collect::<String>());
    let output = nummificate(&query);
    println!("{}", output[0]);
}

fn nummificate(query: &str) -> Vec<i32> {
    // 1. sanitize input for non-digits
    // 2. run aq on input, turning query into its aq numerical equivalent
    // 3. run digitalreducetion on res, saving each round into results vector, until single
    //    digit.
    let mut res = Vec::<i32>::new();
    let mut n = aq(&String::from(query));
    res.push(n);

    while !is_single_digit(&n) {
        n = dreduce(&n);
        res.push(n);
    }
    return res;
}

// Anglobal communications (English) => 
// NOTE: skips characters in query that are not in ALPHANUM
fn aq(query: &String) -> i32 {
    if query.is_empty() { return 0 }
    let mut chars: Vec<_> = query.split("").collect();
    chars.remove(0);
    chars.pop();
    // println!("{:?}", chars);
    let curr = chars.pop().expect("chars is empty!");
    // println!("{:?}", curr);
    let i: usize;
    match ALPHANUM.find(curr) {
        None => return 0 + aq(&chars.join("")),
        Some(index) => i = index,
    }
    return AQ[i] + aq(&chars.join(""))
}

// digital reduction; nine-twin summing
fn dreduce(n: &i32) -> i32 {
    match is_single_digit(&n) {
        true => n.abs(),
        false => n.abs() % 10 + dreduce(&(n/10)),
    }
}

fn is_single_digit(n: &i32) -> bool {
    n.abs() < 10
}

