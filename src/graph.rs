//! A graph, implemented using an array of sets.
//! Parallel edges and self-loops allowed.

use crate::bag::LinkedBag as Bag;
use crate::error::{Algs4Error, InvalidArgument};
use crate::io::In;
use std::fmt;
use std::io::{BufRead, ErrorKind};

pub mod path;
pub use path::*;

/// The `Graph` struct represents an undirected graph of vertices named `0` through `v - 1`.
///
/// It supports the following two primary operations: add an edge to the graph, iterate over all of
/// the vertices adjacent to a vertex.  It also provides methods for returning the degree of a
/// vertex, the number of vertices `v` in the graph, and the number of edges `e` in the graph.
///
/// Parallel edges and self-loops are permitted.  By convention, a self-loop `v-v` appears in the
/// adjacency list of `v` twice and contributes two to the degree of `v`.
///
/// This implementation uses an <em>adjacency-lists representation</em>, which is a vertex-indexed
/// array of `Bag` objects.  It uses &Theta;(`e + v`) space, where `e` is the number of edges and
/// `v` is the number of vertices.  All instance methods take &Theta;(1) time. (Though, iterating
/// over the vertices returned by `adj(usize)` takes time proportional to the degree of the vertex.)
///
/// Constructing an empty graph with `v` vertices takes &Theta;(`v`) time; constructing a graph with
/// `e` edges and `v` vertices takes &Theta;(`e + v`) time.
///
/// For additional documentation, see <a href="https://algs4.cs.princeton.edu/41graph">Section
/// 4.1</a> of <i>Algorithms, 4th Edition</i> by Robert Sedgewick and Kevin Wayne.
#[derive(Clone)]
pub struct Graph {
    v: usize,             // number of vertices
    e: usize,             // number of edges
    adj: Vec<Bag<usize>>, // adjacency lists (require Bag: Clone)
}

impl Graph {
    pub fn new_no_edge(v: usize) -> Self {
        Graph {
            v,
            adj: vec![Bag::new(); v],
            e: 0,
        }
    }

    /// Initializes a graph from the specified input stream.
    pub fn new<T: BufRead>(fileinput: &mut In<T>) -> Result<Self, Algs4Error> {
        let v: usize = read_v(fileinput)?;
        let e: usize = read_e(fileinput)?;
        let mut g = Graph {
            v,
            adj: vec![Bag::new(); v],
            e: 0,
        };
        for _ in 0..e {
            let v: usize = read_edge_vertex(fileinput)?;
            let w: usize = read_edge_vertex(fileinput)?;
            g.add_edge(v, w)?;
        }
        Ok(g)
    }

    /// Adds the undirected edge `v-w` to this graph.
    pub fn add_edge(&mut self, v: usize, w: usize) -> Result<(), InvalidArgument> {
        self.validate_vertex(v)?;
        self.validate_vertex(w)?;
        self.e += 1;
        self.adj[v].add(w);
        self.adj[w].add(v);
        Ok(())
    }

    /// Returns the number of vertices in this graph.
    pub fn count_vertices(&self) -> usize {
        self.v
    }

    /// Returns the number of edges in this graph.
    pub fn count_edges(&self) -> usize {
        self.e
    }

    /// Returns the vertices adjacent to vertex `v`.
    pub fn adj(&self, v: usize) -> Result<impl Iterator<Item = &usize>, InvalidArgument> {
        self.validate_vertex(v)?;
        Ok(self.adj[v].iter())
    }

    /// Returns the degree of vertex `v`.
    pub fn degree(&self, v: usize) -> Result<usize, InvalidArgument> {
        self.validate_vertex(v)?;
        Ok(self.adj[v].len())
    }

    /// Returns a string representation of this graph in DOT format,
    /// suitable for visualization with Graphviz.
    ///
    /// To visualize the graph, install Graphviz (e.g., "brew install graphviz").
    /// Then use one of the graph visualization tools
    ///    - dot    (hierarchical or layer drawing)
    ///    - neato  (spring model)
    ///    - fdp    (force-directed placement)
    ///    - sfdp   (scalable force-directed placement)
    ///    - twopi  (radial layout)
    ///
    /// For example, the following commands will create graph drawings in SVG
    /// and PDF formats
    ///    - dot input.dot -Tsvg -o output.svg
    ///    - dot input.dot -Tpdf -o output.pdf
    ///
    /// To change the graph attributes (e.g., vertex and edge shapes, arrows, colors)
    ///  in the DOT format, see <https://graphviz.org/doc/info/lang.html>
    pub fn to_dot(&self) -> String {
        let mut s = String::new();
        s.push_str("graph {\n");
        s.push_str(
            "node[shape=circle, style=filled, fixedsize=true, width=0.3, fontsize=\"10pt\"]\n",
        );
        let mut self_loops: usize = 0;
        for v in 0..self.v {
            for &w in self.adj[v].iter() {
                if v < w {
                    s.push_str(&format!("{v} -- {w}\n"));
                } else if v == w {
                    // include only one copy of each self loop (self loops will be consecutive)
                    if self_loops % 2 == 0 {
                        s.push_str(&format!("{v} -- {w}\n"));
                    }
                    self_loops += 1;
                }
            }
        }
        s.push_str("}\n");
        s
    }

    fn validate_vertex(&self, v: usize) -> Result<(), InvalidArgument> {
        if v >= self.v {
            Err(InvalidArgument(format!(
                "vertex {} is not between 0 and {}",
                v,
                self.v - 1
            )))
        } else {
            Ok(())
        }
    }
}

impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = format!("{} vertices, {} edges\n", self.v, self.e);
        for v in 0..self.v {
            s.push_str(&format!("{}: ", v));
            for w in self.adj[v].iter() {
                s.push_str(&w.to_string());
                s.push(' ');
            }
            s.push('\n');
        }
        write!(f, "{s}")
    }
}

fn read_x<T: BufRead>(
    fileinput: &mut In<T>,
    invalid_arg_err_msg: &'static str,
    not_found_err_msg: &'static str,
    io_err_msg: &'static str,
) -> Result<usize, Algs4Error> {
    match fileinput.read_int() {
        Ok(x) => Ok(x),
        Err(e) => match e.kind() {
            ErrorKind::InvalidData => {
                Err(Algs4Error::InvalidArgument(invalid_arg_err_msg.to_string()))
            }
            ErrorKind::NotFound => Err(Algs4Error::InvalidArgument(not_found_err_msg.to_string())),
            _ => Err(Algs4Error::InvalidArgument(io_err_msg.to_string())),
        },
    }
}

fn read_v<T: BufRead>(fileinput: &mut In<T>) -> Result<usize, Algs4Error> {
    read_x(
        fileinput,
        "number of vertices in a Graph must be non-negative integer, invalid input format in Graph constructor",
        "number of vertices not found in input, invalid input format in Graph constructor",
        "I/O error when reading number of vertices, invalid input format in Graph constructor",
    )
}

fn read_e<T: BufRead>(fileinput: &mut In<T>) -> Result<usize, Algs4Error> {
    read_x(
        fileinput,
        "number of edges in a Graph must be non-negative integer, invalid input format in Graph constructor",
        "number of edges not found in input, invalid input format in Graph constructor",
        "I/O error when reading number of edges, invalid input format in Graph constructor",
    )
}

fn read_edge_vertex<T: BufRead>(fileinput: &mut In<T>) -> Result<usize, Algs4Error> {
    read_x(
        fileinput,
        "vertex in a Graph must be non-negative integer, invalid input format in Graph constructor",
        "vertex of an edge not found in input, invalid input format in Graph constructor",
        "I/O error when reading a vertex of an edge, invalid input format in Graph constructor",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    // fn cmp_adjacency_lists<'a, T, const N: usize>(a: T, b: [usize; N])
    // where T: Iterator<Item=&'a usize>
    // {
    //     assert_eq!(a.cloned().collect::<HashSet<usize>>(), HashSet::from(b));
    // }
    macro_rules! cmp_adj {
        ($a:expr, $b:expr) => {
            assert_eq!($a.cloned().collect::<HashSet<usize>>(), HashSet::from($b));
        };
    }

    #[test]
    fn graph_basics() -> Result<(), Algs4Error> {
        let mut g = Graph::new_no_edge(13);
        g.add_edge(0, 5)?;
        g.add_edge(4, 3)?;
        g.add_edge(0, 1)?;
        g.add_edge(9, 12)?;
        g.add_edge(6, 4)?;
        g.add_edge(5, 4)?;
        g.add_edge(0, 2)?;
        g.add_edge(11, 12)?;
        g.add_edge(9, 10)?;
        g.add_edge(0, 6)?;
        g.add_edge(7, 8)?;
        g.add_edge(9, 11)?;
        g.add_edge(5, 3)?;
        assert_eq!(g.count_vertices(), 13);
        assert_eq!(g.count_edges(), 13);
        cmp_adj!(g.adj(0).unwrap(), [6, 2, 1, 5]);
        cmp_adj!(g.adj(1).unwrap(), [0]);
        cmp_adj!(g.adj(1).unwrap(), [0]);
        cmp_adj!(g.adj(2).unwrap(), [0]);
        cmp_adj!(g.adj(3).unwrap(), [5, 4]);
        cmp_adj!(g.adj(4).unwrap(), [5, 6, 3]);
        cmp_adj!(g.adj(5).unwrap(), [3, 4, 0]);
        cmp_adj!(g.adj(6).unwrap(), [0, 4]);
        cmp_adj!(g.adj(7).unwrap(), [8]);
        cmp_adj!(g.adj(8).unwrap(), [7]);
        cmp_adj!(g.adj(9).unwrap(), [11, 10, 12]);
        cmp_adj!(g.adj(9).unwrap(), [11, 10, 12]);

        let g2 = g.clone();
        assert_eq!(g2.count_vertices(), 13);
        assert_eq!(g2.count_edges(), 13);
        cmp_adj!(g2.adj(0).unwrap(), [6, 2, 1, 5]);
        cmp_adj!(g2.adj(1).unwrap(), [0]);
        cmp_adj!(g2.adj(1).unwrap(), [0]);
        cmp_adj!(g2.adj(2).unwrap(), [0]);
        cmp_adj!(g2.adj(3).unwrap(), [5, 4]);
        cmp_adj!(g2.adj(4).unwrap(), [5, 6, 3]);
        cmp_adj!(g2.adj(5).unwrap(), [3, 4, 0]);
        cmp_adj!(g2.adj(6).unwrap(), [0, 4]);
        cmp_adj!(g2.adj(7).unwrap(), [8]);
        cmp_adj!(g2.adj(8).unwrap(), [7]);
        cmp_adj!(g2.adj(9).unwrap(), [11, 10, 12]);
        cmp_adj!(g2.adj(9).unwrap(), [11, 10, 12]);
        Ok(())
    }
}
