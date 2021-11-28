/* aq.rs - gematric and digital reduction functions for Anglossic Qabbala (AQ) */
mod tui;

use tui::Tui;
use tui::TuiResult;

use termion::raw::IntoRawMode;
use clap::{App, Arg};
use std::io::{self, Write};

const PROJECT_NAME: &str = "aq";
const VERSION: &str = "0.1.0";
const ABOUT: &str = "deCrypter for Anglobal communications";

fn main() {
    let args = App::new(PROJECT_NAME)
        .version(VERSION)
        .about(ABOUT)
        //.setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("QUERY")
                .help("an alphanumeric-encoded string")
                .index(1),
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

    // Get and lock the stdios.
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    init_and_loop(&mut stdout, args, &query);
}

fn init_and_loop(stdout: &mut io::StdoutLock, args: clap::ArgMatches, init_query: &str) {
    let mut stdout = stdout.into_raw_mode().unwrap();

    // init Tui struct
    let mut tui = Tui::new(init_query);

    // set options
    if args.is_present("t") {
        tui.set_print_trinomes(true);
    }
    // if query; process and return
    if args.is_present("QUERY") {
        tui.print_results(vec![format!("{}", tui.query())]);
        return;
    }

    // start prompt
    write!(stdout, "{} {}\n\r", PROJECT_NAME, VERSION).unwrap();
    let mut state = TuiResult::Ready;
    loop {
        state = match state {
            TuiResult::Ready => tui.run(),
            TuiResult::Quit => return,
            TuiResult::Done(query) => {
                /*
                if let Err(e) = self.write_history(cli.history()) {
                    eprintln!("Could not write query history: {}", e);
                }
                */
                tui.print_results(vec![format!("{}", query)])
            }
        }
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

