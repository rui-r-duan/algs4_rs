use algs4_rs::In;
use algs4_rs::threesum;
use std::env;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input: In = In::new(file_path);
    let a: Vec<i32> = input.read_all_i32()?;

    let now = Instant::now();
    let count = threesum::count(&a);
    println!("elapsed time = {:.3}s", now.elapsed().as_secs_f64());
    println!("{}", count);

    Ok(())
}
