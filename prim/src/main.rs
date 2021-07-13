/* prim.rs - functions for Primitivization (₱)
 * more info elsewhere: zerophilosophy.substack.com/p/qabbalistic-oddments-00
 */
use clap::{Arg, App, AppSettings};
use lazy_static::lazy_static;
#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell, format};
use std::collections::HashMap;
use std::convert::TryInto;
const PROGRAM_NAME: &str = "prim";
const PRIM: &str = "₱";
const I32MAXDIGITS: u32 = 10;

lazy_static! {
    static ref PENTAZYGON_PLUS: Vec<i32> = (0..19).collect();
    static ref NUMONYMS: Vec<String> = vec!["zero", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen",
        "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"]
            .into_iter()
            .map(String::from)
            .collect();
    static ref PLACEHOLDERS: Vec<&'static str> = vec!["hundred", "thousand", "million", "billion"];
    static ref TENS: HashMap<&'static str, i32> = [("twenty", 20), ("thirty", 30), ("fourty", 40), ("fifty", 50),
        ("sixty", 60), ("seventy", 70), ("eighty", 80), ("ninety", 90)].iter().cloned().collect();
}

struct PrimResult {
    prim: i32,
    eng: String,
    pd: i32,
}

fn main() {
    let args = App::new(PROGRAM_NAME)
        .version("0.1.0")
        .about(PRIM)
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("QUERY")
            .help("a word of length < 999")
            .required(true)
            .index(1))
        .get_matches();
    
    let q = args.value_of("QUERY").unwrap();
    primitivize(&q.chars().filter(|&c|c.is_alphabetic()).collect::<String>());
}

// tally, then anglicize; repeat
// EX: walrus => ₱6 => six => ₱3 => three => (₱4 => four)^inf
// "all roads lead to four"
fn primitivize(query: &str) {
    let mut res: Vec::<PrimResult> = Vec::new();
    let mut prim = -1;
    let mut out = String::from(query);
    while prim != 4 {
        prim = tally(&out);
        out = anglicize(&prim);
        res.push(build_prim_result(prim, &out, pd(nummify(&out))));
    }
    print_results(&query, &res);
}

fn build_prim_result(prim: i32, eng: &String, pd: i32) -> PrimResult {
    PrimResult {
        prim,
        eng: eng.to_string(),
        pd,
    }
}

fn print_results(query: &str, prims: &Vec::<PrimResult>) {
    let title = "Primitive Method (₱)";
    println!("+{:-<width$} {} {:-<width$}+", "", title, "", width=query.len()/2);
    println!(" query: '{}' has ₱{:<width$} ", query, prims[0].prim, width=3);
    
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.add_row(row!["PRIM", "ANGL.", "PD"]);
    for res in prims {
        table.add_row(row![format!("₱{}", res.prim), res.eng, res.pd]);
    }
    table.printstd();
    println!("+{:-<width$}+", "", width=query.len() + title.len());
}

// count the letters in a word
// NOTE: ignores whitespace
fn tally(word: &str) -> i32 {
    let mut count = 0;
    for _ in word.chars().filter(|&c|c.is_alphabetic()).collect::<Vec<char>>(){
        count += 1;
    }
    count
}

// the english equivalent of prim
fn anglicize(prim: &i32) -> String {
    let mut out = String::new();
    let mut n = *prim;
    while n > 0 {
        if n % 1000 != 0 {
            out += &translate_three_digits(n % 1000);
        }
        n /= 1000;
    }
    out
}

// the numerical equivalent of query
// query should be a series of space-seperated numonyms (the english numbers)
// supports numbers up to INT32 max
fn nummify(query: &str) -> i32 {
    if query.is_empty() { return 0; }
    let numonyms: Vec<&str> = query.split(" ").collect();
    let mut rem: u32 = numonyms.len().try_into().unwrap();
    if rem > I32MAXDIGITS { panic!("query is larger than {} numonyms", I32MAXDIGITS); }
    let mut res = 0;

    // from left to rightmost digit, convert into numeric and add appropriate power of ten
    for numonym in &numonyms {
        if PLACEHOLDERS.contains(&numonym.trim()) {
            res -= 1;
            continue;
        }
        res += match TENS.contains_key(&numonym.trim()) {
            true => index_into_numonym(&numonym.trim()),
            false => index_into_numonym(&numonym.trim()) * i32::pow(10, rem-1),
        };
        rem -= 1;
    }
    res
}

// calculates the Primitive Discrepancy (PD) of number
fn pd(n: i32) -> i32 { 
    return n - tally(&anglicize(&n));
}

fn index_into_numonym(num: &str) -> i32 {
    match get_num_index(&String::from(num), &NUMONYMS) {
        Some(i) => return PENTAZYGON_PLUS[i],
        None => "" // do nothing
    };

    match TENS.get(&num) {
        Some(&i) => return i,
        None => panic!("index to numonym '{}' not found!", num),
    }
}

fn get_num_index(num: &String, array: &Vec<String>) -> Option<usize> {
    array.iter().position(|x| x == num)
}

// FIXME: only supports up to 999 letters
fn translate_three_digits(n: i32) -> String {
    let mut output = String::new();

    // hundreds place
    if (n / 100) % 10 != 0 {
        output += &hundreds(n);
    }
    
    // tens place
    if (n / 10) % 10 != 0 {
        output += &tens(n%100);
        return output;
    }

    // ones place
    if n % 10 != 0 {
        output += &ones(n%10);
    }
    return output;
}

fn hundreds(n: i32) -> String {
    let mut out = String::new();
    out += ones((n/100)%10);
    out += " hundred ";
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
        5 => out += "fifty",
        6 => out += "sixty",
        7 => out += "seventy",
        8 => out += "eighty",
        9 => out += "ninety",
        _ => panic!("input was not 2-9"),
    }
    if n%10 != 0 {
        out += " ";
        out += ones(n%10);
    }
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

