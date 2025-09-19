/*!
 * $ cargo run --example minpq -- < tinyPQ.txt
 * E A E (6 left on pq)
 */
use algs4_rs::MinPQ;
use algs4_rs::StdIn;

fn main() -> std::io::Result<()> {
    let mut pq = MinPQ::new();
    let mut stdin = StdIn::new();
    while !stdin.is_empty() {
        let item = stdin.read_string()?;
        if item != "-" {
            pq.insert(item);
        } else if !pq.is_empty() {
            print!("{} ", pq.del_min().unwrap());
        }
    }
    println!("({} left on pq)", pq.len());
    Ok(())
}
