/* aq.rs - gematric and digital reduction functions for Anglossic Qabbala (AQ) */
extern crate termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use clap::{App, AppSettings, Arg};
use colored::*;
use std::convert::TryInto;
use std::io::{Read, Write, stdout, stdin};
use std::io::BufRead;

use aq::*;

const PROJECT_NAME: &str = "aq";
const VERSION: &str = "0.1.0";
const ABOUT: &str = "deCrypter for Anglobal communications";

static mut PRINT_HEXTRINOME: bool = false;
const BACKSPACE: char = 8u8 as char;

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
        .arg(
            Arg::with_name("t")
                .short("t")
                .multiple(false)
                .help("print hex trinomes"),
        )
        .get_matches();

    let query: String = match args.value_of("QUERY") {
        None => String::new(),
        Some(query) => sanitize_query(&query),
    };

    if args.is_present("t") {
        unsafe {
            PRINT_HEXTRINOME = true;
        }
    }

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

    // init all streams
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut keys = stdin.keys();

    loop {
        print_and_clear(&mut buffer);
        print!("> ");
        stdout.flush().unwrap();

        let c = keys.next().unwrap().unwrap();

        match c {
            Key::Char('q') => break,
            Key::Esc => break,
            Key::Left => println!("←"),
            Key::Right => println!("→"),
            Key::Up => println!("↑"),
            Key::Down => println!("↓"),
            Key::Backspace => println!("×"),
            _ => {}
        }
        // stdout.flush().unwrap();

        /*
         match b {
                // Quit
                b'q' => return,
                // Clear the screen
                b'c' => write!(stdout, "{}", termion::clear::All),
                // Set red color
                b'r' => write!(stdout, "{}", termion::color::Fg(termion::color::Rgb(124, 252, 0))),
                // Write it to stdout.
                a => write!(stdout, "{}", a),
            }
            .unwrap();

             stdout.flush().unwrap();
             */

        /*
        stdin
            .read_line(&mut buffer)
            .expect("error: unable to read user input");
        buffer = buffer.trim().to_uppercase();

        if buffer.is_empty() || is_quit(&buffer) {
            break;
        } else if buffer == "^[[D" {
            stdout.write_all(b"\x1B[1D");
        }

        print!("{:?}", buffer);
        print_and_clear(&buffer);
        */
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

fn print_and_clear(buffer: &mut String) {
    if !buffer.is_empty() {
        print_results(&buffer);
    }
    buffer.clear();
}

fn print_results(buffer: &String) {
    print!("{}", buffer);
    for res in &nummificate(&sanitize_query(&buffer)) {
        print!(" -> {}", res);
    }
    print!("\n");

    if unsafe {PRINT_HEXTRINOME} {
        let mut i = 0;
        let mut trinomes = Vec::<u8>::new();
        let mut title_printed = false;
        for c in buffer.chars().filter(|&c|c.is_alphanumeric()).collect::<String>().chars() {
            let trinome: u8 = nummificate(&c.to_string())[0].try_into().unwrap();
            trinomes.push(trinome);
            i += 1;
            if i % 3 == 0 {
                if !title_printed {
                    println!("{:->width$} THE IRON LAW OF SIX {:->width$}", "", "", width=40);
                }
                title_printed = true;
                print_hex_trinomes(&trinomes);
                trinomes.clear();
            }
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
