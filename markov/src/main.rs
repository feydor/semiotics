use std::fs;
use structopt::StructOpt;
use anyhow::{Result};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("markov/src/markov.h"); 
        // fn markov(text: &CxxString, words: i32) -> UniquePtr<CxxString>;
        fn markovn(text: &CxxString, words: i32, N: i32) -> UniquePtr<CxxString>;
    }
}

#[derive(StructOpt, Debug)]
struct Cli {

    // the path of the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    // the number of words to generate
    #[structopt(default_value = "100")]
    word_count: i32,

    // markov ngram
    #[structopt(default_value = "4")]
    ngram: i32,
}

// Run NLG on a text file and output the results
fn main() -> Result<()> {
    let args = Cli::from_args();
    let text = fs::read_to_string(args.path)?;

    cxx::let_cxx_string!(cxx_text = text);
    let generated = ffi::markovn(&cxx_text, args.word_count, args.ngram);
    println!("{}", *generated);

    Ok(())
}
