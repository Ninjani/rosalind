use crate::algorithmic_heights::r5_ddeg::make_adjacency_matrix;
use crate::utils;
use hashbrown::{HashMap, HashSet};
use std::collections::VecDeque;

/// Breadth-First Search
///
/// Given: A simple directed graph with n≤10^3 vertices in the edge list format.
///
/// Return: An array D[1..n] where D[i] is the length of a shortest path from the vertex 1 to the vertex i (D[1]=0). If i is not reachable from 1 set D[i] to −1.
pub fn rosalind_bfs() {
    let (num_nodes, _, edges) = utils::read_edge_list(&utils::input_from_file(
        "data/algorithmic_heights/rosalind_bfs.txt",
    ));
    let adjacency_matrix = make_adjacency_matrix(&edges, true);
    for node in 1..=num_nodes {
        print!(
            "{} ",
            bfs_length(&adjacency_matrix, 1, node)
                .map(|x| x as isize)
                .unwrap_or(-1)
        );
    }
}

fn get_path_length(node: usize, path: &HashMap<usize, Option<usize>>) -> usize {
    let mut node = node;
    let mut length = 0;
    while let Some(n) = path[&node] {
        node = n;
        length += 1;
    }
    length
}

/// Finds length of shortest path from start_node to end_node using breadth-first search
fn bfs_length(
    adjacency_matrix: &HashMap<usize, Vec<usize>>,
    start_node: usize,
    end_node: usize,
) -> Option<usize> {
    let mut open_set = VecDeque::new();
    let mut closed_set = HashSet::new();
    open_set.push_back(start_node);
    let mut path = HashMap::new();
    path.insert(start_node, None);
    while !open_set.is_empty() {
        let subtree_root = open_set.pop_front().unwrap();
        if subtree_root == end_node {
            return Some(get_path_length(subtree_root, &path));
        }
        if let Some(edge_list) = adjacency_matrix.get(&subtree_root) {
            for child in edge_list {
                if !closed_set.contains(child) && !open_set.contains(child) {
                    path.insert(*child, Some(subtree_root));
                    open_set.push_back(*child);
                }
            }
        }
        closed_set.insert(subtree_root);
    }
    None
}
