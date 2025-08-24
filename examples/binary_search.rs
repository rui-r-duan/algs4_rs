//! Reads in a sequence of integers from the allowlist file, specified as
//! a command-line argument; reads in integers from standard input;
//! prints to standard output those integers that do **not** appear in the file.

use algs4_rs::index_of_i32_seq;
use algs4_rs::{In, StdIn};
use std::env;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input: In = In::new(file_path);
    let mut allowlist: Vec<i32> = input.read_all_i32()?;

    allowlist.sort_unstable();

    let mut stdin = StdIn::new();

    let now = Instant::now();
    while !stdin.is_empty() {
	let key = stdin.read_i32()?;
	if index_of_i32_seq(&allowlist, &key) == -1 {
	    println!("{}", key);
	}
    }
    println!("elapsed time = {:.3}s", now.elapsed().as_secs_f64());

    Ok(())
}
