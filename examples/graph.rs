/*!
 * A graph, implemented using an array of sets.
 * Parallel edges and self-loops allowed.
 *
 * Data files:  https://algs4.cs.princeton.edu/41graph/tinyG.txt
 *              https://algs4.cs.princeton.edu/41graph/mediumG.txt
 *              https://algs4.cs.princeton.edu/41graph/largeG.txt
 *
 * $ cargo run --example graph -- tinyG.txt
 * 13 vertices, 13 edges
 * 0: 6 2 1 5
 * 1: 0
 * 2: 0
 * 3: 5 4
 * 4: 5 6 3
 * 5: 3 4 0
 * 6: 0 4
 * 7: 8
 * 8: 7
 * 9: 11 10 12
 * 10: 9
 * 11: 9 12
 * 12: 11 9
 *
 * $ cargo run --example graph -- Graph mediumG.txt
 * 250 vertices, 1273 edges
 * 0: 225 222 211 209 204 202 191 176 163 160 149 114 97 80 68 59 58 49 44 24 15
 * 1: 220 203 200 194 189 164 150 130 107 72
 * 2: 141 110 108 86 79 51 42 18 14
 * ...
 *
 * Feel free to feed invalid input to test various error scenarios.
 */

use algs4_rs::{FileIn, Graph, error::Algs4Error};
use std::env;

fn main() -> Result<(), Algs4Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut input = FileIn::new(file_path)?;
    let g = Graph::new(&mut input)?;
    println!("{:?}", g);
    println!("in DOT format:");
    println!("{}", g.to_dot());
    Ok(())
}
