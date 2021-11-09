// vivisix - symbolic mathematical computator
extern crate vivi;
use crate::vivi::vivi::vivi::*;

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
        repl(&config);
    }

    let mut evaluator = Vivi::new(&config.query);

    let result = run(config, &mut evaluator).unwrap_or_else(|err| {
        println!("Problem evaluating query: {}", err);
        process::exit(1);
    });
    evaluator.display();
}

fn repl(config: &Config) {
    println!("{} {}", PROJECT_NAME, VERSION);
    let mut line = String::new();
    let mut evaluator = Vivi::new(&"1 + 1".to_string());
    loop {
        print!("> ");
        line = read!("{}\n");
        if line.len() == 0 {
            process::exit(0);
        }
        evaluator.expr(&line);
        evaluator.display();
    }
}

fn run(config: Config, evaluator: &mut Vivi) -> Result<(), &str> {
    if config.flags.len() == 0 {
        return Ok(evaluator.eval()); // x + 2x -> x ADD 2 MUL x
    }

    for flag in config.flags {
        return match flag.as_str() {
            "-d" => Ok(evaluator.derive()),
            _ => Err("flag not found"),
        }
    }
    Err("END OF RUN")
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Self, &str> {
        if args.len() < 2 {
            return Ok(Config { query: "".to_string(), flags: [].to_vec(), none: true })
        }

        let query = args[1].clone();
        let flags = args[2..].iter().map(|s| s.to_string()).collect();

        Ok(Config { query, flags, none: false })
    }
}