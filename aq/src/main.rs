/* aq.rs - gematric and digital reduction functions for Anglossic Qabbala (AQ) */
use toei::toei::*;
use clap::{App, Arg};

const PROJECT_NAME: &str = "aq";
const VERSION: &str = "0.1.0";
const ABOUT: &str = "deCrypter for Anglobal communications";

fn main() {
    let args = App::new(PROJECT_NAME)
        .version(VERSION)
        .about(ABOUT)
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

    let query = sanitize_query(args.value_of("QUERY").unwrap_or_default());

    let aq_closure = |s: &str| {
        let v = aq::nummificate(&s.to_uppercase());
        let mut out: String = format!("{} -> {}", s, v[0]);
        if args.is_present("t") {
            out += "\n";
            out += &aq::trinomes(&s);
        }
        return out;
    };

    if args.is_present("QUERY") {
        println!("{}", aq_closure(&query));
    }

    let mut t = Toei::new();
    println!("{} {}", PROJECT_NAME, VERSION);
    loop {
        match t.run(aq_closure) {
            ToeiResult::Done(x) => t.print_result(&x),
            ToeiResult::Quit => break,
            _ => (),
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

