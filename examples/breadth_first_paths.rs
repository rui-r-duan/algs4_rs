/*!
 *  Data files:   https://algs4.cs.princeton.edu/41graph/tinyCG.txt
 *                https://algs4.cs.princeton.edu/41graph/tinyG.txt
 *                https://algs4.cs.princeton.edu/41graph/mediumG.txt
 *                https://algs4.cs.princeton.edu/41graph/largeG.txt
 *
 *  Run breadth-first search on an undirected graph.
 *  Runs in O(E + V) time.
 *
 *  $ cargo run --example graph -- tinyCG.txt
 *  6 vertices, 8 edges
 *  0: 2 1 5
 *  1: 0 2
 *  2: 0 1 3 4
 *  3: 5 4 2
 *  4: 3 2
 *  5: 3 0
 *
 *  $ cargo run --example breadth_first_paths -- tinyCG.txt 0
 *  0 to 0 (0):  0
 *  0 to 1 (1):  0-1
 *  0 to 2 (1):  0-2
 *  0 to 3 (2):  0-2-3
 *  0 to 4 (2):  0-2-4
 *  0 to 5 (1):  0-5
 *
 *  $ cargo run --example breadth_first_paths -- largeG.txt 0
 *  0 to 0 (0):  0
 *  0 to 1 (418):  0-932942-474885-82707-879889-971961-...
 *  0 to 2 (323):  0-460790-53370-594358-780059-287921-...
 *  0 to 3 (168):  0-713461-75230-953125-568284-350405-...
 *  0 to 4 (144):  0-460790-53370-310931-440226-380102-...
 *  0 to 5 (566):  0-932942-474885-82707-879889-971961-...
 *  0 to 6 (349):  0-932942-474885-82707-879889-971961-...
 */

use algs4_rs::Algs4Error;
use algs4_rs::{BreadthFirstPaths, FileIn, Graph};
use std::env;

fn main() -> Result<(), Algs4Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut input = FileIn::new(file_path)?;
    let g = Graph::new(&mut input)?;
    match args[2].parse::<usize>() {
        Ok(s) => {
            let bfs = BreadthFirstPaths::new(&g, s)?;
            for v in 0..g.count_vertices() {
                if bfs.has_path_to(v)? {
                    print!("{} to {} ({}):  ", s, v, bfs.dist_to(v)?);
                    for x in bfs.path_to(v).expect("v should be a valid vertex") {
                        if x == s {
                            print!("{x}");
                        } else {
                            print!("-{x}");
                        }
                    }
                    println!();
                } else {
                    println!("{} to {} (-):  not connected", s, v);
                }
            }
            Ok(())
        }
        Err(_) => Err(Algs4Error::InvalidArgument(
            "source vertex should be a valid usize".to_string(),
        )),
    }
}
