//! tetractys.rs - triangular number generation; method of binomial coefficients
use std::convert::TryInto;
// use lazy_static::lazy_static;
/*
lazy_static! {
    static ref GRID: Vec<u8> = Vec::<u8>::with_capacity(100);
}
*/

fn main() {
    let n: u32 = std::env::args().nth(1).expect("no number given").parse().expect("argument not a number");
    let size: u32 = match n % 2 == 0 {
         true => (n+1)*(n+1),
         false => n*n
    };

    let grid: Vec<bool> = vec![false; size.try_into().unwrap()];
    let tetrads = tetrads(n);
    print_tetrads(&tetrads);
    // print_grid(&grid);
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
    if n < 2 {
        return 1; 
    }
    return n + triangular(n-1);
}

fn print_tetrads(tetrads: &Vec<u32>) {
    let base: u32 = *tetrads.last().unwrap();
    for n in tetrads {
        // first print spaces
        for _ in *n..=base {
            print!(" ");
        }

        for _ in 0..*n {
            print!("{}", "· ");
        }
        print!("\n");
    }
}

fn print_grid(grid: &Vec<bool>) {
    let dim = (grid.len() as f64).sqrt();
    println!("{:?}", dim);
    for i in 0..grid.len() {
        if i as f64 % dim == 0.0 {
            print!("\n");
        }

        match grid[i] {
            true => print!("•"),
            false => print!("·")
        }
    }
}
