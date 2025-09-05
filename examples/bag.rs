///////////////////////////////////////////////////////////////////////////////
// $ more tobe.txt
// to be or not to - be - - that - - - is
//
// $ cargo run --example bag < tobe.txt
// size of bag = 14
// is
// -
// -
// -
// that
// -
// -
// be
// -
// to
// not
// or
// be
// to
///////////////////////////////////////////////////////////////////////////////

use algs4_rs::StdIn;
use algs4_rs::ResizingBag as Bag;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let mut bag = Bag::new();
    let mut stdin = StdIn::new();
    let now = Instant::now();
    while !stdin.is_empty() {
        let item = stdin.read_string()?;
        bag.add(item);
    }

    println!("size of bag = {}", bag.len());
    for s in bag.iter() {
        println!("{s}");
    }
    println!("elapsed time = {:.3}s", now.elapsed().as_secs_f64());

    Ok(())
}
