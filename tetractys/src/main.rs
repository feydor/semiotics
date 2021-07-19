//! tetractys.rs - triangular number generation; method of binomial coefficients
use std::convert::TryInto;

fn main() {
    let n: u32 = std::env::args().nth(1).expect("no number given").parse().expect("argument not a number");
    let tetrads = tetrads(n);
    print_tetrads(&tetrads);
    println!("The {}th triangular number is: {}", n, triangular(n));
}

// returns the constituent parts of the nth triangular number
fn tetrads(n: u32) -> Vec<u32> {
    let mut parts = Vec::<u32>::with_capacity(n.try_into().unwrap());
    for i in 1..n+1 {
        parts.push(i);
    }
    parts
}

// returns the 1st to nth triangular numbers
fn tri_upto(n: u32) -> Vec<u32> {
    let mut series = Vec::<u32>::with_capacity(n.try_into().unwrap());
    for i in (1..n+1).rev() {
        series.push(triangular(i));
    }
    series
}

// returns the nth triangular number
// T=1,3,6,10,15,21,etc
fn triangular(n: u32) -> u32 {
    if n < 2 { return 1; }
    return n + triangular(n-1);
}

fn print_tetrads(tetrads: &Vec<u32>) {
    let base: u32 = *tetrads.last().unwrap();
    // println!(" r Σ");
    for n in tetrads {
        // first print (base-n) spaces
        print!("{:>2} ", *n);
        for _ in *n..=base {
            print!("·");
        }

        // then print n dots
        for i in 0..*n {
            print!("●");

            if i != *n-1 {
                print!(" ");
            }
        }

        // closing spaces
        for _ in *n..=base {
            print!("·");
        }
        print!("\n");
    }
}

