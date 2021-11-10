// vivisix - symbolic mathematical computator
extern crate vivi;
use crate::vivi::vivi::vivi::*;

use std::env;
use std::process;
use text_io::read;

#[derive(Debug)]
struct Config {
    query: String,
    flags: Vec<String>,
    none: bool,
}

const PROJECT_NAME: &str = "vivi";
const VERSION: &str = "0.1.0";

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("query: {}", config.query);
    println!("flags: {:?}", config.flags);

    if config.none {
        repl();
    }

    let mut evaluator = Vivi::new(&config.query);

    run(config, &mut evaluator).unwrap_or_else(|err| {
        println!("Problem evaluating query: {}", err);
        process::exit(1);
    });
}

fn repl() {
    println!("{} {}", PROJECT_NAME, VERSION);
    let evaluator = Vivi::new(&"".to_string());

    loop {
        print!("> ");
        let line: String = read!("{}\n");
        if line.len() == 0 {
            process::exit(0);
        }
                   
        evaluator.differentiate(&sanitize_query(&line), "x");
    }
}

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn run(config: Config, evaluator: &mut Vivi) -> Result<(), &str> {
    if config.flags.len() == 0 {
        // TEST queries
        let e1 = vec_of_strings!["x"];
        let e2 = vec_of_strings!["2"];
        let s1 = evaluator.make_sum(e1, e2);
        let f1 = vec_of_strings!["x"];
        let f2 = vec_of_strings!["5"];
        let s2 = evaluator.make_sum(f1, f2);
        let sum = evaluator.make_sum(s1, s2);
        println!("{:?}", sum);
    }

    for flag in config.flags {
        return match flag.as_str() {
            "-dx" => Ok(evaluator.differentiate(&config.query, "x")),
            s if s == "-h" || s == "--help" => Ok(print_help()),
            s if s == "-V" || s == "--version" => Ok(println!("{} {}", PROJECT_NAME, VERSION)),
            _ => Err("flag not found"),
        }
    }
    Err("END OF RUN")
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Self, &str> {
        if args.len() == 1 {
            return Ok(Config { query: "".to_string(), flags: [].to_vec(), none: true })
        } else if args.len() == 2 {
            return Ok(Config { query: sanitize_query(&args[1]), flags: [].to_vec(), none: false })
        }

        let query = sanitize_query(&args[1]);
        let flags = args[2..].iter().map(|s| s.to_string()).collect();

        Ok(Config { query, flags, none: false })
    }
}

fn sanitize_query(query: &str) -> String {
    query.chars()
         .filter(|c| c.is_alphanumeric() || c.is_ascii_punctuation() || c.is_whitespace())
         .map(|c| c.to_string().to_lowercase())
         .collect::<String>()
}

fn print_help() {
    println!("{}", "
    USAGE:
        vivi [FLAGS] [QUERY]
    
    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information
    
    ARGS:
        <QUERY>    a mathematically-encoded string
     
    EXAMPLE:
        vivi 'x * x + 22' -dx
        'x + x'

    ");
}