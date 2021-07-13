/* prim.rs - functions for Primitivization (₱)
 * more info elsewhere: zerophilosophy.substack.com/p/qabbalistic-oddments-00
 */
use clap::{Arg, App, AppSettings};
const PROGRAM_NAME: &str = "prim";
const PRIM: &str = "₱";

fn main() {
    let args = App::new(PROGRAM_NAME)
        .version("0.1.0")
        .about(PRIM)
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("QUERY")
            .help("a word")
            .required(true)
            .index(1))
        .get_matches();
    
    let query = args.value_of("QUERY").unwrap();
    primitivize(&query);
}

fn primitivize(query: &str) {
    let mut prim = -1;
    let mut out = String::from(query);
    out = out.chars().filter(|&c|c.is_alphabetic()).collect::<String>();
    while prim != 0 {
        prim = tally(&out);
        out = anglicize(&prim);
        // print_result(&out, &prim);
        println!("{} == {} == {}{}", query, out, PRIM, prim);
    }
}

// NOTE: remove whitespace before passing
fn tally(word: &str) -> i32 {
    let mut count = 0;
    for _ in word.chars(){
        count += 1;
    }
    count
}

fn anglicize(prim: &i32) -> String {
    let mut out = String::new();
    let mut itr = 0;
    let mut n = *prim;
    while n > 0 {
        if n % 1000 != 0 {
            out += &translate_three_digits(n % 1000);
        }
        n /= 1000;
        itr += 1;
    }
    out
}

fn translate_three_digits(n: i32) -> String {
    let mut output = String::new();

    // hundreds place
    if n / 100 != 0 {
        output += &hundreds(n);
    }
    
    // tens place
    if n / 10 != 0 {
        output += &tens(n % 100);
    }

    // ones place
    // output += &ones(n % 10);
    return output;
}

fn hundreds(n: i32) -> String {
    let mut out = String::new();
    out += ones(n % 10);
    out += " hundred";
    return out;
}

// EX: 87 => "eighty", 11 => "eleven"
fn tens(n: i32) -> String {
    let mut out = String::new();
    match n/10 {
        1 => {
            out += teens(n%10);
            return out;
        },
        2 => out += "twenty",
        3 => out += "thirty",
        4 => out += "fourty",
        5 => out += "fivety",
        6 => out += "sixty",
        7 => out += "seventy",
        8 => out += "eighty",
        9 => out += "ninety",
        _ => panic!("input was not 2-9"),
    }
    out += " ";
    out += ones(n%10);
    return out;
}

fn teens(n: i32) -> &'static str {
    match n {
        0 => "ten",
        1 => "eleven",
        2 => "twelve",
        3 => "thirteen",
        4 => "fourteen",
        5 => "fifteen",
        6 => "sixteen",
        7 => "seventeen",
        8 => "eighteen",
        9 => "nineteen",
        _ => panic!("input was not 0-9"),
    }
}

fn ones(n: i32) -> &'static str {
    match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => panic!("input was not 0-9"),
    }
}

