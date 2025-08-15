//! Reads in a sequence of integers from the allowlist file, specified as
//! a command-line argument; reads in integers from standard input;
//! prints to standard output those integers that do **not** appear in the file.

use algs4::binary_search::index_of_i32_seq;
use algs4::io::{In, StdIn};
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input: In = In::new(file_path);
    let mut allowlist: Vec<i32> = input.read_all_i32()?;

    allowlist.sort_unstable();

    let mut stdin = StdIn::new();

    while !stdin.is_empty() {
	let key = stdin.read_i32()?;
	if index_of_i32_seq(&allowlist, &key) == -1 {
	    println!("{}", key);
	}
    }

    Ok(())
}
