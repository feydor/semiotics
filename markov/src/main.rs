use std::fs;
use structopt::StructOpt;
use anyhow::{Result};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("markov/src/markov.h");
        
        fn markov(text: &CxxString, words: i32) -> UniquePtr<CxxString>;
    }
}

// Run Natural Language Generation on a text file and output the results
#[derive(StructOpt, Debug)]
struct Cli {
    // the path of the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let text = fs::read_to_string(args.path)?;

    cxx::let_cxx_string!(cxx_text = text);
    let generated = ffi::markov(&cxx_text, 100);
    println!("{}", *generated);

    Ok(())
}
