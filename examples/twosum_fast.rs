use algs4_rs::In;
use algs4_rs::error::Algs4Error;
use algs4_rs::twosum_fast;
use std::env;
use std::time::Instant;

fn main() -> Result<(), Algs4Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input: In = In::new(file_path);
    let mut a: Vec<i32> = input.read_all_i32()?;

    let now = Instant::now();
    let count = twosum_fast::count(&mut a)?;
    println!("elapsed time = {:.3}s", now.elapsed().as_secs_f64());
    println!("{}", count);

    Ok(())
}
