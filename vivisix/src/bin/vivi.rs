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

    println!("args: {}", args.len());
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
    evaluator.display();
}

fn repl() {
    println!("{} {}", PROJECT_NAME, VERSION);
    let mut evaluator = Vivi::new(&"".to_string());

    loop {
        print!("> ");
        let line: String = read!("{}\n");
        if line.len() == 0 {
            process::exit(0);
        }
                            
        evaluator.parse_expr(&sanitize_query(&line));
        evaluator.display();
    }
}

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn run(config: Config, evaluator: &mut Vivi) -> Result<(), &str> {
    if config.flags.len() == 0 {
        // return Ok(evaluator.eval()); // x + 2x -> x ADD 2 MUL x
        let e1 = vec_of_strings!["2", "+", "2"];
        let e2 = vec_of_strings!["5", "+", "5"];
        let sum = evaluator.make_sum(&e1, &e2);
        println!("{:?}", sum);
    }

    for flag in config.flags {
        return match flag.as_str() {
            // "-d" => Ok(evaluator.differentiate("x")),
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