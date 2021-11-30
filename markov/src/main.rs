// use quicli::prelude::*;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashSet;
use structopt::StructOpt;
use anyhow::{Context, Result};

// Run Natural Language Generation on a text file and output the results
#[derive(StructOpt, Debug)]
struct Cli {
    // the path of the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    println!("{:?}", args);
    let file = File::open(&args.path)
        .with_context(|| format!("could not read file {:?}", &args.path))?;
    let reader = BufReader::new(file);
    let mut i = 0;
    let k = 2; // look at k words at a time
    let mut distinct = HashSet::new();

    for line in reader.lines() {
        for word in line.unwrap().split_whitespace() {
            let w = word.to_string();
            distinct.insert(w); // sets ignore duplicates
            i += 1;
        }
    }
    println!("total: {}", i);
    println!("distinct: {}", distinct.len());

    Ok(())
}
