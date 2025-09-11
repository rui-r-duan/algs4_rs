use super::Graph;
use super::path::{BreadthFirstPaths, DepthFirstPaths};
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

fn tiny_graph() -> Graph {
    let mut g = Graph::new_no_edge(13);
    g.add_edge(0, 5).unwrap();
    g.add_edge(4, 3).unwrap();
    g.add_edge(0, 1).unwrap();
    g.add_edge(9, 12).unwrap();
    g.add_edge(6, 4).unwrap();
    g.add_edge(5, 4).unwrap();
    g.add_edge(0, 2).unwrap();
    g.add_edge(11, 12).unwrap();
    g.add_edge(9, 10).unwrap();
    g.add_edge(0, 6).unwrap();
    g.add_edge(7, 8).unwrap();
    g.add_edge(9, 11).unwrap();
    g.add_edge(5, 3).unwrap();
    g
}

fn check_tiny_graph(g: &Graph) {
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
}

fn tiny_connected_graph() -> Graph {
    let mut g = Graph::new_no_edge(6);
    g.add_edge(0, 5).unwrap();
    g.add_edge(2, 4).unwrap();
    g.add_edge(2, 3).unwrap();
    g.add_edge(1, 2).unwrap();
    g.add_edge(0, 1).unwrap();
    g.add_edge(3, 4).unwrap();
    g.add_edge(3, 5).unwrap();
    g.add_edge(0, 2).unwrap();
    g
}

#[test]
fn graph_basics() {
    let g = tiny_graph();
    check_tiny_graph(&g);

    let g2 = g.clone();
    check_tiny_graph(&g2);
}

#[test]
fn test_dfs() {
    let g = tiny_connected_graph();
    let dfs = DepthFirstPaths::new(&g, 0).unwrap();
    assert_eq!(dfs.path_to(0).unwrap(), [0]);
    assert_eq!(dfs.path_to(1).unwrap(), [0, 2, 1]);
    assert_eq!(dfs.path_to(2).unwrap(), [0, 2]);
    assert_eq!(dfs.path_to(3).unwrap(), [0, 2, 3]);
    assert_eq!(dfs.path_to(4).unwrap(), [0, 2, 3, 4]);
    assert_eq!(dfs.path_to(5).unwrap(), [0, 2, 3, 5]);
    assert!(dfs.path_to(6).is_err());
}

#[test]
fn test_bfs() {
    let g = tiny_connected_graph();
    let bfs = BreadthFirstPaths::new(&g, 0).unwrap();
    assert_eq!(bfs.path_to(0).unwrap(), [0]);
    assert_eq!(bfs.path_to(1).unwrap(), [0, 1]);
    assert_eq!(bfs.path_to(2).unwrap(), [0, 2]);
    assert_eq!(bfs.path_to(3).unwrap(), [0, 2, 3]);
    assert_eq!(bfs.path_to(4).unwrap(), [0, 2, 4]);
    assert_eq!(bfs.path_to(5).unwrap(), [0, 5]);
    assert!(bfs.path_to(6).is_err());
}
