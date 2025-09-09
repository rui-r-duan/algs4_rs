use crate::error::InvalidArgument;
use crate::graph::Graph;
use crate::stack::linkedstack::LinkedStack as Stack;

/// Finds paths from a source vertex `s` to every other vertex in an undirected graph, using
/// depth-first search.
///
/// The constructor takes &Theta;(<em>V</em> + <em>E</em>) time in the worst case, where <em>V</em>
/// is the number of vertices and <em>E</em> is the number of edges.
///
/// Each instance method takes &Theta;(1) time.
///
/// It uses &Theta;(<em>V</em>) extra space (not including the graph).
///
/// For additional documentation, see <a href="https://algs4.cs.princeton.edu/41graph">Section
/// 4.1</a> of <i>Algorithms, 4th Edition</i> by Robert Sedgewick and Kevin Wayne.
pub struct DepthFirstPaths {
    marked: Vec<bool>,   // marked[v] = is there an s-v path?
    edge_to: Vec<usize>, // edge_to[v] = last edge on s-v path
    s: usize,            // source vertex
}

impl DepthFirstPaths {
    pub fn new(g: &Graph, s: usize) -> Result<Self, InvalidArgument> {
        let v = g.count_vertices();
        validate_vertex(s, v)?;
        let mut paths = DepthFirstPaths {
            marked: vec![false; v],
            edge_to: vec![0; v],
            s,
        };
        Self::dfs(&mut paths, g, s);
        Ok(paths)
    }

    // Precondition: `v` is a valid vertex
    fn dfs(&mut self, g: &Graph, v: usize) {
        self.marked[v] = true;
        for &w in g.adj(v).expect("v should be a valid vertex") {
            if !self.marked[w] {
                self.edge_to[w] = v;
                self.dfs(g, w);
            }
        }
    }

    pub fn has_path_to(&self, v: usize) -> Result<bool, InvalidArgument> {
        self.validate_vertex(v)?;
        Ok(self.marked[v])
    }

    pub fn path_to(&self, v: usize) -> Result<Vec<usize>, InvalidArgument> {
        if !self.has_path_to(v)? {
            Ok(Vec::new())
        } else {
            let mut path = Stack::new();
            let mut x = v;
            while x != self.s {
                path.push(x);
                x = self.edge_to[x];
            }
            path.push(self.s);
            Ok(path.iter().cloned().collect())
        }
    }

    fn validate_vertex(&self, s: usize) -> Result<(), InvalidArgument> {
        validate_vertex(s, self.marked.len())
    }
}

fn validate_vertex(s: usize, count_vertices: usize) -> Result<(), InvalidArgument> {
    if s >= count_vertices {
        Err(InvalidArgument(format!(
            "vertex {} is not between 0 and {}",
            s,
            count_vertices - 1
        )))
    } else {
        Ok(())
    }
}
