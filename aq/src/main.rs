/* aq.rs - gematric and digital reduction functions for Anglossic Qabbala (AQ) */
extern crate termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::{raw::IntoRawMode, cursor::DetectCursorPos};
use clap::{App, AppSettings, Arg};
use colored::*;
use std::convert::TryInto;
use std::io::{self, Read, Write};
use std::fmt::Display;
use std::io::StdoutLock;
// use std::io::BufRead;

const PROJECT_NAME: &str = "aq";
const VERSION: &str = "0.1.0";
const ABOUT: &str = "deCrypter for Anglobal communications";

struct Tui<R> {
    state: ConsoleState,
    query: String,           // the current query
    line: InputLine,
    curr_pos: Coord,         // the cursor's position
    term_size: Coord,        // the terminal's size
    prompt: String,          // the default prompt
    do_print_trinomes: bool, // optional trinome printing
    stdin: R,                // standard input
}
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone)]
enum ConsoleState {
    Start,
    Typing,
    HistoryUp,
    HistoryDown,
    Done,
    Quit
}

pub enum TuiResult {
    // Suggestions(String),
    Done(String),
    Quit,
    Ready,
}

#[derive(Default, Clone)]
struct InputLine {
    cursor: usize,
    line: String,
}

impl InputLine {
    pub fn from_string(line: String) -> Self {
        InputLine {
            cursor: line.len(),
            line,
        }
    }

    pub fn into_string(self) -> String {
        self.line
    }

    pub fn as_str(&self) -> &str {
        &self.line
    }

    pub fn len(&self) -> usize {
        self.line.len()
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

     pub fn insert(&mut self, ch: char) {
        // end of line insert (push)
        if self.cursor > self.line.len() {
            self.line.push(ch);
            self.cursor = self.line.len() + 1;
        } else {
            // inbetween line insert
            self.line = self
                .line
                .chars()
                .take(self.cursor) // chars up to cursor
                .chain(std::iter::once(ch)) // plus character
                .chain(self.line.chars().skip(self.cursor)) // plus chars after cursor
                .collect();
            self.cursor += 1;
        }
    }

    pub fn cursor_left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    pub fn cursor_right(&mut self) {
        self.cursor = self.line.len().min(self.cursor + 1);
    }

    pub fn backspace(&mut self) {
        if self.cursor == 0 { return; }
        if self.cursor > self.line.len() {
            self.line.pop();
        } else {
            self.line = self
                .line
                .chars()
                .take(self.cursor - 1)
                .chain(self.line.chars().skip(self.cursor))
                .collect()
        }
        self.cursor -= 1;
    }
}

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

    // Get and lock the stdios.
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let stderr = io::stderr();
    let mut stderr = stderr.lock();

    // go into raw mode
    // let stdout = stdout.into_raw_mode().unwrap();

    init_and_loop(stdin, &mut stdout, args, query);
}

fn init_and_loop<R: Read>(stdin: R, stdout: &mut StdoutLock, args: clap::ArgMatches, init_query: String) {
    // init Tui struct
    let size = termion::terminal_size().unwrap();
    let mut stdout = stdout.into_raw_mode().unwrap();
    let curr_pos = stdout.cursor_pos().unwrap();

    let  mut tui = Tui {
        query: init_query,
        state: ConsoleState::Start,
        line: InputLine::default(),
        curr_pos: Coord {
            x: curr_pos.0 as usize,
            y: curr_pos.1 as usize,
        },
        term_size: Coord {
            x: size.0 as usize,
            y: size.1 as usize,
        },
        prompt: "> ".to_owned(),
        do_print_trinomes: false,
        stdin: stdin.keys(),
    };

    // set options
    if args.is_present("t") {
        tui.do_print_trinomes = true;
    }
    // handle query
    if !args.is_present("i") {
        tui.print_result_and_clear();
        return;
    }

    // start prompt
    write!(stdout, "{}\n\r{}\n\r", PROJECT_NAME, VERSION).unwrap();
    let mut state = TuiResult::Ready;
    loop {
        state = match state {
            TuiResult::Ready => tui.run(),
            TuiResult::Quit => return,
            TuiResult::Done(query) => {
                /*
                if let Err(e) = self.save_history(cli.history()) {
                    eprintln!("Could not save query history: {}", e);
                }
                */
                tui.print_results(vec![format!("{}", query)])
                /*
                match run_query(&query) {
                    Ok(results) => tui.print_results(results),
                    Err(e) => tui.print_results(vec![format!("{}", e)]),
                }
                */
            }
        }
    }
}

impl <R: Iterator<Item=Result<Key, std::io::Error>>> Tui<R> {
    fn run(&mut self) -> TuiResult {
        //self.print_result_and_clear();
        // let mut buffer = String::new();

        // self.print_prompt();
        self.state = ConsoleState::Start;

        // go into raw mode (destroy at end of run)
        let stdout = io::stdout().into_raw_mode().unwrap();
        let mut stdout = stdout.lock();
        loop {
            self.render(&mut stdout);
            self.state = match self.state {
                ConsoleState::Start => Some(ConsoleState::Typing),
                ConsoleState::Typing => {
                    let key = self.stdin.next().unwrap().unwrap();
                    typing(key, &mut self.line)
                }
                ConsoleState::Done => {
                    let query = std::mem::take(&mut self.line).into_string();
                    //self.history.save(query.clone());
                    // Put it in the right state for next time
                    //self.state = ConsoleState::Typing;
                    if query.is_empty() {
                        return TuiResult::Quit;
                    } else {
                        return TuiResult::Done(query);
                    }
                }
                ConsoleState::Quit => {
                    return TuiResult::Quit;
                }
                _ => Some(self.state)
            }
            .unwrap_or(self.state);
        }
    }

    fn render(&mut self, stdout: &mut StdoutLock) {
        let (_,y) = stdout.cursor_pos().unwrap();
        match self.state {
            ConsoleState::Start => {
                write!(
                    stdout,
                    "{}{}>\r\n{}",
                    termion::cursor::Goto(1, y),
                    termion::clear::CurrentLine,
                    termion::cursor::Goto(self.line.cursor() as u16 + 3, y - 1),
                )
                .unwrap();
            }
            ConsoleState::Typing => {
                write!(
                    stdout,
                    "{}{}> {}\r\n{}{}",
                    termion::cursor::Goto(1, y),
                    termion::clear::CurrentLine,
                    self.line.as_str(),
                    termion::clear::CurrentLine,
                    termion::cursor::Goto(self.line.cursor() as u16 + 3, y),
                )
                .unwrap();
            }
            ConsoleState::Done => {
                write!(
                    stdout,
                    "\r\n{}{}",
                    termion::clear::CurrentLine,
                    termion::cursor::Goto(self.line.cursor() as u16 + 3, y)
                )
                .unwrap();
            }
            ConsoleState::Quit => {
                write!(stdout, "\r\n{}", termion::clear::AfterCursor).unwrap();
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }

    fn print_results(&mut self, results: Vec<String>) -> TuiResult {
        {
            let stdout = io::stdout().into_raw_mode().unwrap();
            stdout.suspend_raw_mode().unwrap();
            let mut stdout = stdout.lock();
            write!(&mut stdout, "\r\n").unwrap();

            // print query followed by aq results
            for item in results {
                write!(&mut stdout, "{}", item.to_uppercase()).unwrap();

                for res in &aq::nummificate(&item.to_uppercase()) {
                    write!(stdout, " -> {}", res).unwrap();
                }
                write!(stdout, "\r\n");
            }
            stdout.flush().unwrap();
        }
        let stdout = io::stdout().into_raw_mode().unwrap();
        stdout.activate_raw_mode().unwrap();
        self.state = ConsoleState::Start;
        TuiResult::Ready
    }

    fn print_result_and_clear(&mut self) {
        if self.query.is_empty() {
            return;
        }

        let stdout = io::stdout().into_raw_mode().unwrap();
        let mut stdout = stdout.lock();

        write!(stdout, "{}", self.query).unwrap();
        for res in &aq::nummificate(&self.query.to_uppercase()) {
            write!(stdout, " -> {}", res).unwrap();
        }
        write!(stdout, "\n\r").unwrap();

        if self.do_print_trinomes {
            self.print_trinomes();
        }
        self.query.clear();
    }

    fn print_trinomes(&mut self) {
        let mut i = 0;
        let mut trinomes = Vec::<u8>::new();
        let mut title_printed = false;
        for c in self.query.chars().filter(|&c|c.is_alphanumeric()).collect::<String>().chars() {
            let trinome: u8 = aq::nummificate(&c.to_string())[0].try_into().unwrap();
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

fn typing(key: Key, line: &mut InputLine) -> Option<ConsoleState> {
    match key {
        Key::Esc | Key::Ctrl('c') => return Some(ConsoleState::Quit),
        Key::Char(ch) => {
            match ch {
                '\n' => return Some(ConsoleState::Done),
                // '\t' => return Some(ConsoleState::GetSuggestions)
                _ => line.insert(ch),
            }
        }
        Key::Left => line.cursor_left(),
        Key::Right => line.cursor_right(),
        // Key::Up => return Some(ConsoleState::HistoryUp),
        // Key::Down => return Some(ConsoleState::HistoryDown),
        Key::Backspace => {
            if line.len() > 0 {
                line.backspace();
            }
        }
        _ => {
            // write!(stdout, "{:?}", k).unwrap();
        }
    }
    None
}

// removes non-alphanumerics and converts to uppercase
fn sanitize_query(q: &str) -> String {
    return q
        .chars()
        .filter(|&c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .to_uppercase();
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
