//! tui.rs - tui (terminal user interface) container abstraction with state and result enums
mod history;
mod input_line;
use history::History;
use input_line::InputLine;
use std::convert::TryInto;
use termion::event::Key;
//use termion::color;
use termion::{raw::IntoRawMode, cursor::DetectCursorPos};
use termion::input::TermRead;
use std::io::{self, Write};

pub struct Tui {
    state: ConsoleState,
    query: String,           // the current query
    line: InputLine,
    history: History,
    prompt: String,          // the default prompt
    do_print_trinomes: bool, // optional trinome printing
}

#[derive(Debug, Copy, Clone)]
pub enum ConsoleState {
    Start,
    Typing,
    HistoryUp,
    HistoryDown,
    Matches,
    Done,
    Quit
}

pub enum TuiResult {
    Done(String),
    Quit,
    Ready,
}

impl Tui {
    pub fn new(init_query: String) -> Self {
        Tui {
            query: init_query,
            state: ConsoleState::Start,
            line: InputLine::default(),
            history: History::default(),
            prompt: "> ".to_owned(),
            do_print_trinomes: false,
        }
    }

    pub fn run(&mut self) -> TuiResult {
        // let mut buffer = String::new();
        self.state = ConsoleState::Start;

        // go into raw mode (destroy at end of run)
        let stdout = io::stdout().into_raw_mode().unwrap();
        let mut stdout = stdout.lock();
        let mut keys = io::stdin().keys();
        loop {
            self.render(&mut stdout); // mutates state
            self.state = match self.state {
                ConsoleState::Start => Some(ConsoleState::Typing),
                ConsoleState::Typing => {
                    let key = keys.next().unwrap().unwrap();
                    typing(key, &mut self.line)
                }
                ConsoleState::HistoryUp => {
                    self.history.up();
                    self.line = InputLine::from_string(
                        self.history.current_query().unwrap_or_default().to_string()
                    );
                    Some(ConsoleState::Typing)
                }
                ConsoleState::HistoryDown => {
                    self.history.down();
                    self.line = InputLine::from_string(
                        self.history.current_query().unwrap_or_default().to_string()
                    );
                    Some(ConsoleState::Typing)
                }
                ConsoleState::Matches => Some(ConsoleState::Start),
                ConsoleState::Done => {
                    let query = std::mem::take(&mut self.line).into_string();
                    let number = aq::nummificate(&query.to_uppercase())[0]; // take first result

                    // save query in history
                    self.history.save((number, query.clone()));

                    // Put it in the right state for next time
                    if query.is_empty() {
                        return TuiResult::Quit;
                    } else {
                        return TuiResult::Done(query);
                    }
                }
                ConsoleState::Quit => {
                    return TuiResult::Quit;
                }
            }
            .unwrap_or(self.state);
        }
    }

    fn render(&mut self, stdout: &mut io::StdoutLock) {
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
            ConsoleState::Matches => {
                let matches = match self.history.matches() {
                    None => { eprint!{"\r\n No Matches return"}; return; },
                    Some(m) => m,
                };

                let mut new_line = String::new();
                for (n, vec) in &matches {
                    for query in vec.iter() {
                        new_line += &(query.to_string() + &" = ".to_string());
                    }
                    new_line += &(n.to_string() + "\r\n");
                }

                self.line = InputLine::from_string(new_line);

                write!(
                    stdout,
                    "{}{}> {}\r\n{}{}",
                    termion::cursor::Goto(1, y),
                    termion::clear::CurrentLine,
                    self.line.as_str(),
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

    // TODO: results should be a Vec of tuples Vec<(i32, String)> with aq precalculated
    pub fn print_results(&mut self, results: Vec<String>) -> TuiResult {
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
                write!(stdout, "\r\n").unwrap();
            }
            stdout.flush().unwrap();
        }
        let stdout = io::stdout().into_raw_mode().unwrap();
        stdout.activate_raw_mode().unwrap();
        self.state = ConsoleState::Start;
        TuiResult::Ready
    }

    pub fn print_trinomes(&mut self) {
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

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn set_print_trinomes(&mut self, b: bool) {
        self.do_print_trinomes = b;
    }
}

// handles immediate user input
// moves cursor, returns control signals, adds/removes characters from the line buffer
fn typing(key: Key, line: &mut InputLine) -> Option<ConsoleState> {
    match key {
        Key::Esc | Key::Ctrl('c') => return Some(ConsoleState::Quit),
        Key::Char(ch) => {
            match ch {
                '\n' => return Some(ConsoleState::Done),
                '#' => return Some(ConsoleState::Matches),
                _ => line.insert(ch),
            }
        }
        Key::Left => line.cursor_left(),
        Key::Right => line.cursor_right(),
        Key::Up => return Some(ConsoleState::HistoryUp),
        Key::Down => return Some(ConsoleState::HistoryDown),
        Key::Backspace => {
            if line.len() > 0 {
                line.backspace();
            }
        }
        Key::Delete => {
            if line.len() > 0 {
                line.delete_key();
            }
        }
        _ => {}
    }
    None
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
    termion::color::Rgb(trinomes[0]*SCALE, trinomes[1]*SCALE, trinomes[2]*SCALE);
    for _ in 0..6 {
        print!("{} ", s);
    }
    print!{"\n"};
}
