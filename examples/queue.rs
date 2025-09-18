/*!
 * $ more tobe.txt
 * to be or not to - be - - that - - - is
 *
 * $ cargo run --example queue -- < tobe.txt
 * to be or not to be (2 left on queue)
 */

use algs4_rs::LinkedQueue as Queue;
use algs4_rs::StdIn;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let mut qu = Queue::new();
    let mut stdin = StdIn::new();
    let now = Instant::now();
    while !stdin.is_empty() {
        let item = stdin.read_string()?;
        if item != "-" {
            qu.enqueue(item);
        } else if !qu.is_empty() {
            print!("{} ", qu.dequeue().unwrap());
        }
    }
    println!("({} left on queue)", qu.len());
    println!("elapsed time = {:.3}s", now.elapsed().as_secs_f64());
    Ok(())
}
