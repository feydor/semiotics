/* aq.rs - gematric and digital reduction functions for Anglossic Qabbala (AQ) */
use std::io;
use std::io::Write;
use lazy_static::lazy_static;
use clap::{Arg, App, AppSettings};
const PROJECT_NAME: &str = "aq";
const VERSION: &str = "0.1.0";
const ABOUT: &str = "deCrypter for Anglobal communications";
const ALPHANUM: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

lazy_static! {
    static ref AQ: Vec<i32> = (0..36).collect();
}

fn main() {
    let args = App::new(PROJECT_NAME)
        .version(VERSION)
        .about(ABOUT)
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("QUERY")
            .help("an alphanumeric-encoded string")
            .index(1))
        .arg(Arg::with_name("i")
            .short("i")
            .multiple(false)
            .help("start interactive prompt"))
        .get_matches();

    let query: String = match args.value_of("QUERY") {
        None => String::new(),
        Some(query) => sanitize_query(&query),
    };

    if args.is_present("i") {
        start_prompt(&query)
    } else {
        print_results(&query);
    }
}

fn start_prompt(initial: &str) {
    println!("{}\n{}", PROJECT_NAME, VERSION);
    let mut buffer = match initial.is_empty() {
        true => String::new(),
        false => String::from(initial),
    };
    let stdin = io::stdin();
    
    loop {
        if !buffer.is_empty() { print_results(&buffer); }
        buffer.clear();

        print!{"> "};
        io::stdout().flush().unwrap();
        stdin.read_line(&mut buffer).expect("error: unable to read user input");
        buffer = buffer.trim().to_uppercase();
        if buffer.is_empty() || is_quit(&buffer) {
            break;
        }
        print_results(&buffer);
        buffer.clear();
    }
}

// removes non-alphanumerics and converts to uppercase
fn sanitize_query(q: &str) -> String {
    return q.chars().filter(|&c|c.is_alphanumeric() || c.is_whitespace()).collect::<String>().to_uppercase();
}

fn print_results(buffer: &String) {
    print!("{}", buffer);
    for res in &nummificate(&sanitize_query(&buffer)) {
        print!(" == {}", res);
    }
    print!("\n");
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

fn is_quit(q: &str) -> bool {
    match q {
        "q" => true,
        "Q" => true,
        _ => false,
    }
}
