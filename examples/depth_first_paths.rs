/*!
 *  Data files:   https://algs4.cs.princeton.edu/41graph/tinyCG.txt
 *                https://algs4.cs.princeton.edu/41graph/tinyG.txt
 *                https://algs4.cs.princeton.edu/41graph/mediumG.txt
 *                https://algs4.cs.princeton.edu/41graph/largeG.txt
 *
 *  Run depth-first search on an undirected graph.
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
 *  $ cargo run --example depth_first_paths -- tinyCG.txt 0
 *  0 to 0:  0
 *  0 to 1:  0-2-1
 *  0 to 2:  0-2
 *  0 to 3:  0-2-3
 *  0 to 4:  0-2-3-4
 *  0 to 5:  0-2-3-5
 */

use algs4_rs::error::Algs4Error;
use algs4_rs::{DepthFirstPaths, FileIn, Graph};
use std::env;

fn main() -> Result<(), Algs4Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut input = FileIn::new(file_path)?;
    let g = Graph::new(&mut input)?;
    match args[2].parse::<usize>() {
        Ok(s) => {
            let dfs = DepthFirstPaths::new(&g, s)?;
            for v in 0..g.count_vertices() {
                if dfs.has_path_to(v)? {
                    print!("{} to {}:  ", s, v);
                    for x in dfs.path_to(v).expect("v should be a valid vertex") {
                        if x == s {
                            print!("{x}");
                        } else {
                            print!("-{x}");
                        }
                    }
                    println!();
                } else {
                    println!("{} to {}:  not connected", s, v);
                }
            }
            Ok(())
        }
        Err(_) => Err(Algs4Error::InvalidArgument(
            "source vertex should be a valid usize".to_string(),
        )),
    }
}
