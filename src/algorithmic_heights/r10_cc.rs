use crate::algorithmic_heights::r5_ddeg::make_adjacency_matrix;
use crate::utils;
use std::collections::HashMap;
use std::iter::repeat;

/// Connected Components
///
/// Given: A simple graph with n≤10^3 vertices in the edge list format.
///
/// Return: The number of connected components in the graph.
pub fn rosalind_cc() {
    let (num_nodes, _, edges) = utils::read_edge_list(&utils::input_from_file(
        "data/algorithmic_heights/rosalind_cc.txt",
    ));
    let adjacency_matrix = make_adjacency_matrix(&edges, false);
    println!(
        "{}",
        count_connected_components(num_nodes, &adjacency_matrix)
    );
}

/// Finds number of connected components
pub fn count_connected_components<S: ::std::hash::BuildHasher>(
    num_nodes: usize,
    adjacency_matrix: &HashMap<usize, Vec<usize>, S>,
) -> usize {
    let mut visited = repeat(false).take(num_nodes).collect::<Vec<_>>();
    let mut num_cc = 0;
    for node in 1..=num_nodes {
        if !visited[node - 1] {
            num_cc += 1;
            dfs(&mut visited, node, &adjacency_matrix);
        }
    }
    num_cc
}

/// Depth-first search
pub fn dfs<S: ::std::hash::BuildHasher>(
    visited: &mut [bool],
    node: usize,
    adjacency_matrix: &HashMap<usize, Vec<usize>, S>,
) {
    let mut stack = Vec::new();
    stack.push(node);
    visited[node - 1] = true;
    while !stack.is_empty() {
        let subtree_root = stack.pop().unwrap();
        if !visited[subtree_root - 1] {
            visited[subtree_root - 1] = true;
        }
        if let Some(edge_list) = adjacency_matrix.get(&subtree_root) {
            for child in edge_list {
                if !visited[*child - 1] {
                    stack.push(*child);
                }
            }
        }
    }
}
