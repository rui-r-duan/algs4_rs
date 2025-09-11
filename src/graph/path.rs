use crate::error::InvalidArgument;
use crate::graph::Graph;
use crate::queue::resizingqueue::ResizingQueue as Queue;
use crate::stack::resizingstack::ResizingStack as Stack;

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

fn validate_vertices(sources: &[usize], count_vertices: usize) -> Result<(), InvalidArgument> {
    if sources.len() == 0 {
        return Err(InvalidArgument("zero vertices".to_string()));
    }
    for &v in sources {
        validate_vertex(v, count_vertices)?;
    }
    Ok(())
}

pub struct BreadthFirstPaths {
    marked: Vec<bool>,   // marked[v] = is there an s-v path?
    edge_to: Vec<usize>, // edge_to[v] = previous edge on shortest s-v path
    dist_to: Vec<usize>, // dist_to[v] = number of edges on shortest s-v path
}

const INFINITY: usize = usize::MAX;

impl BreadthFirstPaths {
    pub fn new(g: &Graph, s: usize) -> Result<Self, InvalidArgument> {
        let v = g.count_vertices();
        validate_vertex(s, v)?;
        let mut paths = BreadthFirstPaths {
            marked: vec![false; v],
            edge_to: vec![0; v],
            dist_to: vec![INFINITY; v],
        };
        Self::bfs(&mut paths, g, s);
        debug_assert!(Self::check(&paths, g, s));
        Ok(paths)
    }

    pub fn new_multiple_sources(g: &Graph, sources: &[usize]) -> Result<Self, InvalidArgument> {
        let v = g.count_vertices();
        validate_vertices(sources, v)?;
        let mut paths = BreadthFirstPaths {
            marked: vec![false; v],
            edge_to: vec![0; v],
            dist_to: vec![INFINITY; v],
        };
        Self::bfs_multiple_sources(&mut paths, g, sources);
        Ok(paths)
    }

    // breadth-first search from a single source
    //
    // Preconditions:
    // - elements in `dist_to` are set to INFINITY.
    fn bfs(&mut self, g: &Graph, s: usize) {
        assert!(self.dist_to.iter().all(|x| *x == INFINITY));
        let mut q: Queue<usize> = Queue::new();
        self.dist_to[s] = 0;
        self.marked[s] = true;
        q.enqueue(s);
        while !q.is_empty() {
            let v = q
                .dequeue()
                .expect("non-empty queue should have some elements");
            for &w in g.adj(v).expect("v should be a valid vertex") {
                if !self.marked[w] {
                    self.edge_to[w] = v;
                    self.dist_to[w] = self.dist_to[v] + 1;
                    self.marked[w] = true;
                    q.enqueue(w);
                }
            }
        }
    }

    // breadth-first search from multiple sources
    fn bfs_multiple_sources(&mut self, g: &Graph, sources: &[usize]) {
        assert!(self.dist_to.iter().all(|x| *x == INFINITY));
        let mut q: Queue<usize> = Queue::new();
        for &s in sources {
            self.marked[s] = true;
            self.dist_to[s] = 0;
            q.enqueue(s);
        }
        while !q.is_empty() {
            let v = q
                .dequeue()
                .expect("non-empty queue should have some elements");
            for &w in g.adj(v).expect("v should be a valid vertex") {
                if !self.marked[w] {
                    self.edge_to[w] = v;
                    self.dist_to[w] = self.dist_to[v] + 1;
                    self.marked[w] = true;
                    q.enqueue(w);
                }
            }
        }
    }

    pub fn has_path_to(&self, v: usize) -> Result<bool, InvalidArgument> {
        self.validate_vertex(v)?;
        Ok(self.marked[v])
    }

    pub fn dist_to(&self, v: usize) -> Result<usize, InvalidArgument> {
        self.validate_vertex(v)?;
        Ok(self.dist_to[v])
    }

    pub fn path_to(&self, v: usize) -> Result<Vec<usize>, InvalidArgument> {
        if !self.has_path_to(v)? {
            Ok(Vec::new())
        } else {
            let mut path = Stack::new();
            let mut x = v;
            while self.dist_to[x] != 0 {
                path.push(x);
                x = self.edge_to[x];
            }
            path.push(x);
            Ok(path.iter().cloned().collect())
        }
    }

    // Check optimality conditions for single source.
    fn check(&self, g: &Graph, s: usize) -> bool {
        // check that the distance of s = 0
        if self.dist_to[s] != 0 {
            eprintln!("distance of source {} to itself = {}", s, self.dist_to[s]);
            return false;
        }

        // check that for each edge v-w dist[w] <= dist[v] + 1
        // provided v is reachable from s
        for v in 0..g.count_vertices() {
            for &w in g.adj(v).expect("v should be a valid vertex") {
                if self.has_path_to(v).unwrap() != self.has_path_to(w).unwrap() {
                    eprintln!("edge {}-{}", v, w);
                    eprintln!("has_path_to({}) = {}", v, self.has_path_to(v).unwrap());
                    eprintln!("has_path_to({}) = {}", w, self.has_path_to(w).unwrap());
                    return false;
                }
                if self.has_path_to(v).unwrap() && self.dist_to[w] > self.dist_to[v] + 1 {
                    eprintln!("edge {}-{}", v, w);
                    eprintln!("dist_to[{}] = {}", v, self.dist_to[v]);
                    eprintln!("dist_to[{}] = {}", w, self.dist_to[w]);
                }
            }
        }

        // check that v = edge_to[w] satisfies dist_to[w] = dist_to[v] + 1
        // provided v is reachable from s
        for w in 0..g.count_vertices() {
            if !self.has_path_to(w).unwrap() || w == s {
                continue;
            }
            let v = self.edge_to[w];
            if self.dist_to[w] != self.dist_to[v] + 1 {
                eprintln!("shortest path edge {}-{}", v, w);
                eprintln!("dist_to[{}] = {}", v, self.dist_to[v]);
                eprintln!("dist_to[{}] = {}", w, self.dist_to[w]);
                return false;
            }
        }

        true
    }

    fn validate_vertex(&self, s: usize) -> Result<(), InvalidArgument> {
        validate_vertex(s, self.marked.len())
    }
}
