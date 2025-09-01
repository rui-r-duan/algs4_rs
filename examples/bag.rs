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

use algs4_rs::Bag;
use algs4_rs::StdIn;

fn main() -> std::io::Result<()> {
    let mut bag = Bag::new();
    let mut stdin = StdIn::new();
    while !stdin.is_empty() {
        let item = stdin.read_string()?;
        bag.add(item);
    }

    println!("size of bag = {}", bag.len());
    for s in bag.iter() {
        println!("{s}");
    }

    Ok(())
}
