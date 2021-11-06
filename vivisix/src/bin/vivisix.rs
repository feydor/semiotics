// vivisix - symbolic mathematical computator
extern crate vivisix;
use crate::vivi::*;

use std::env;
use std::process;
use std::error::Error;
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
        repl(config);
    }

    let result = run(config).unwrap_or_else(|err| {
        println!("Problem evaluating query: {}", err);
        process::exit(1);
    });
    printer::print_expr(&result);
}

fn repl(config: Config) {
    println!("{} {}", PROJECT_NAME, VERSION);
    let mut line = String::new();
    loop {
        print!("> ");
        line = read!("{}\n");
        if line.len() == 0 {
            process::exit(0);
        }
        let expr = vivi::eval(&line);
        printer::print_expr(&expr);
    }
}

fn run(config: Config) -> Result<(), &str> {
    if config.flags.len() == 0 {
        return Ok(vivi::eval(&config.query));
    }

    for flag in config.flags {
        return match flag.as_str() {
            "-d" => Ok(vivi::deriv(&config.query)),
            _ => Err("flag not found"),
        }
    }
    Err("END OF RUN")
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Self, &str> {
        if args.len() < 1 {
            return Ok(Config { query: "".to_string(), flags: [].to_vec(), none: true })
        }

        let query = args[1].clone();
        let flags = args[2..].iter().map(|s| s.to_string()).collect();

        Ok(Config { query, flags, none: false })
    }
}