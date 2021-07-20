/* aq.rs - gematric and digital reduction functions for Anglossic Qabbala (AQ) */
extern crate termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::{raw::IntoRawMode, cursor::DetectCursorPos};
use clap::{App, AppSettings, Arg};
use colored::*;
use std::convert::TryInto;
use std::io::{self, Read, Write};
// use std::io::BufRead;

const PROJECT_NAME: &str = "aq";
const VERSION: &str = "0.1.0";
const ABOUT: &str = "deCrypter for Anglobal communications";

struct Tui<R, W: Write> {
    state: ConsoleState,
    query: String,           // the current query
    line: InputLine,
    curr_pos: Coord,         // the cursor's position
    term_size: Coord,        // the terminal's size
    prompt: String,          // the default prompt
    do_print_trinomes: bool, // optional trinome printing
    stdin: R,                // standard input
    stdout: W,               // standard output
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
    let stdout = stdout.into_raw_mode().unwrap();

    init(stdout, stdin, args, query);
}

fn init<R: Read, W: Write>(mut stdout: W, stdin: R, args: clap::ArgMatches, init_query: String) {
    let size = termion::terminal_size().unwrap();
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
        stdout: stdout,
        stdin: stdin.keys(),
    };

    // set options
    if args.is_present("t") {
        tui.do_print_trinomes = true;
    }

    // start prompt or handle query
    if args.is_present("i") {
        tui.start_prompt();
    } else {
        tui.print_result_and_clear();
    }
}

impl <R: Iterator<Item=Result<Key, std::io::Error>>, W: Write> Tui<R, W> {
    fn start_prompt(&mut self) {
        write!(self.stdout, "{}\n\r{}\n\r", PROJECT_NAME, VERSION).unwrap();
        self.print_result_and_clear();
        // let mut buffer = String::new();

        self.print_prompt();
        loop {
            // read a single byte from stdin
            let b = self.stdin.next().unwrap().unwrap();

            match b {
                Key::Esc | Key::Ctrl('q') | Key::Ctrl('z') => return,
                Key::Backspace => {
                    // move cursor left, print space, curr char from buffer
                    self.cursor_left();
                    self.delete_at_cursor();
                    self.write_str(" ");
                    self.refresh_input_prompt();
                }
                Key::Left => {
                    self.cursor_left();
                },
                Key::Right => {
                    // cursor can be one space after the query and prompt
                    if self.curr_pos.x < self.prompt.len() + self.query.len() + 1 {
                        self.cursor_right();
                    }
                },
                Key::Char('\n') => {
                    self.cursor_newline_return();
                    self.print_result_and_clear();
                    self.cursor_newline_return();
                    self.print_prompt();
                },

                Key::Char(ch) => {
                    // write character, move cursor, add to buffer
                    self.insert_into_query(ch);
                    // self.refresh_input_prompt();
                    //self.replace_at_query(ch);
                    self.write_str(&ch.to_string());
                },
                _ => {},
            }
            self.stdout.flush().unwrap();
        }
    }

    fn render(&mut self) {
        let (_,y) = self.stdout.cursor_pos().unwrap();
        match self.state {
            ConsoleState::Start => {
                write!(
                    self.stdout,
                    "{}{}>\r\n{}",
                    termion::cursor::Goto(1, y),
                    termion::clear::CurrentLine,
                    termion::cursor::Goto(self.line.cursor() as u16 + 3, y - 1),
                )
                .unwrap();
            }
            ConsoleState::Typing => {
                write!(
                    self.stdout,
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
                    self.stdout,
                    "\r\n{}{}",
                    termion::clear::CurrentLine,
                    termion::cursor::Goto(self.line.cursor() as u16 + 3, y)
                )
                .unwrap();
            }
            ConsoleState::Quit => {
                write!(self.stdout, "\r\n{}", termion::clear::AfterCursor).unwrap();
            }
            _ => {}
        }
        self.stdout.flush().unwrap();
    }

    fn print_result_and_clear(&mut self) {
        if self.query.is_empty() {
            return;
        }
        write!(self.stdout, "{}", self.query).unwrap();
        for res in &aq::nummificate(&self.query.to_uppercase()) {
            write!(self.stdout, " -> {}", res).unwrap();
        }
        write!(self.stdout, "\n\r").unwrap();

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

    fn cursor_left(&mut self) {
        if self.curr_pos.x > self.prompt.len() + 1 {
            self.curr_pos.x -= 1;
        }
        write!(self.stdout,
            "{}",
            termion::cursor::Goto(self.curr_pos.x as u16,
                self.curr_pos.y as u16)
        ).unwrap();
    }

    fn cursor_right(&mut self) {
        if self.curr_pos.x < self.term_size.x {
            self.curr_pos.x += 1;
        }
        write!(self.stdout,
            "{}",
            termion::cursor::Goto(self.curr_pos.x as u16,
                self.curr_pos.y as u16)
        ).unwrap();
    }

    fn delete_at_cursor(&mut self) {
        if (self.curr_pos.x > self.prompt.len() + 1) {
            self.query.remove(self.curr_pos.x - (self.prompt.len() + 1));
        }
        write!(self.stdout,
            "{}",
            termion::cursor::Goto(self.curr_pos.x as u16,
                self.curr_pos.y as u16)
        ).unwrap();
    }

    fn write_str(&mut self, s: &str) {
        write!(self.stdout, "{}", s).unwrap();
        for _ in 0..s.len() {
            self.cursor_right();
        }
    }

    fn insert_into_query(&mut self, ch: char) {
        // account for prompt and space at begining of line
        self.query.insert(self.curr_pos.x-1-self.prompt.len(), ch);
    }

    fn replace_at_query(&mut self, ch: char) {
        if self.curr_pos.x > self.prompt.len() + 1 {
            self.query.remove(self.curr_pos.x-1-self.prompt.len());
        }
        self.insert_into_query(ch);
    }

    fn cursor_newline_return(&mut self) {
        self.curr_pos.y += 1;
        self.curr_pos.x = 1;

        write!(self.stdout,
            "\n{}",
            termion::cursor::Goto(self.curr_pos.x as u16,
                self.curr_pos.y as u16)
        ).unwrap();
    }

    fn refresh_input_prompt(&mut self) {
        let old_x = self.curr_pos.x-1;
        let old_y = self.curr_pos.y;
        self.clear_line();
        self.print_prompt();
        let query_copy = String::from(self.query.as_str());
        self.write_str(&query_copy);
        write!(self.stdout,
            "{}",
            termion::cursor::Goto(old_x as u16,
                old_y as u16)
        ).unwrap();
    }

    fn clear_line(&mut self) {
        write!(self.stdout, "{}", termion::clear::CurrentLine).unwrap();
        self.curr_pos.x = 1;
        write!(self.stdout,
            "{}",
            termion::cursor::Goto(self.curr_pos.x as u16,
                self.curr_pos.y as u16)
        ).unwrap();
    }

    fn print_prompt(&mut self) {
        write!(self.stdout, "{}", self.prompt).unwrap();
        self.stdout.flush().unwrap();
        for _ in 0..self.prompt.len() {
            self.cursor_right();
        }
    }
}

/*
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
*/

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

fn is_quit(q: &str) -> bool {
    match q {
        "q" => true,
        "Q" => true,
        _ => false,
    }
}
