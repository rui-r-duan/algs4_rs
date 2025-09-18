/*!
 * $ cargo run --example maxpq -- < tinyPQ.txt
 * Q X P (6 left on pq)
 */
use algs4_rs::MaxPQ;
use algs4_rs::StdIn;

fn main() -> std::io::Result<()> {
    let mut pq = MaxPQ::new();
    let mut stdin = StdIn::new();
    while !stdin.is_empty() {
        let item = stdin.read_string()?;
        if item != "-" {
            pq.insert(item);
        } else if !pq.is_empty() {
            print!("{} ", pq.del_max().unwrap());
        }
    }
    println!("({} left on pq)", pq.len());
    Ok(())
}
