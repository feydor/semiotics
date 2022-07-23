/* aw - interactive AQ nummifier */
use clap::{App, AppSettings, Arg};
use std::io;
use std::io::Write;
use colored::*;
use std::convert::TryInto;
use libaq;
const PROJECT_NAME: &str = "aq";
const VERSION: &str = "0.1.0";
const ABOUT: &str = "deCrypter for Anglobal communications";

fn main() {
    let args = App::new(PROJECT_NAME)
        .version(VERSION)
        .about(ABOUT)
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("QUERY")
                .help("an alphanumeric-encoded string")
                .index(1),
        )
        .arg(
            Arg::with_name("i")
                .short("i")
                .multiple(false)
                .help("start interactive prompt"),
        )
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
        if !buffer.is_empty() {
            print_results(&buffer);
        }
        buffer.clear();

        print! {"> "};
        io::stdout().flush().unwrap();
        stdin
            .read_line(&mut buffer)
            .expect("error: unable to read user input");
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
    return q
        .chars()
        .filter(|&c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .to_uppercase();
}

fn print_results(buffer: &String) {
    print!("{}", buffer);
    for res in &libaq::nummificate(&sanitize_query(&buffer)) {
        print!(" -> {}", res);
    }
    print!("\n");

    println!("{:->width$} THE IRON LAW OF SIX {:->width$}", "", "", width=40);
    let mut i = 0;
    let mut trinomes = Vec::<u8>::new();
    for c in buffer.chars().filter(|&c|c.is_alphanumeric()).collect::<String>().chars() {
        let trinome: u8 = libaq::nummificate(&c.to_string())[0].try_into().unwrap();
        trinomes.push(trinome);
        i += 1;
        if i % 3 == 0 {
            print_hex_trinomes(&trinomes);
            trinomes.clear();
        }
    }
}

// prints the hex trinome in color, using itself
fn print_hex_trinomes(trinomes: &Vec<u8>) {
    if trinomes.len() < 3 {
        panic!("trinomes must be hex.");
    }

    let mut s = String::from(" ");
    for t in trinomes {
        s.push_str(format!("{:#04X} ", t).as_str());
    }

    // println!("--Hex Trinomes--");
    const SCALE: u8 = 4;
    for _ in 0..6 {
        print!("{} ", s.on_truecolor(trinomes[0]*SCALE, trinomes[1]*SCALE, trinomes[2]*SCALE));
    }
    print!{"\n"};
}

fn is_quit(q: &str) -> bool {
    match q {
        "q" => true,
        "Q" => true,
        _ => false,
    }
}
