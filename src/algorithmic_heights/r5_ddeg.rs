use crate::algorithmic_heights::r3_deg::get_degrees;
use crate::utils;
use std::collections::HashMap;

/// Get adjacency matrix from list of edges
pub fn make_adjacency_matrix<T: Eq + Copy + ::std::hash::Hash>(
    edges: &[(T, T)],
    directed: bool,
) -> HashMap<T, Vec<T>> {
    let mut adjacency_matrix = HashMap::new();
    for (node_1, node_2) in edges {
        {
            let edge_list_1 = adjacency_matrix.entry(*node_1).or_insert_with(Vec::new);
            edge_list_1.push(*node_2);
        }
        if !directed {
            let edge_list_2 = adjacency_matrix.entry(*node_2).or_insert_with(Vec::new);
            edge_list_2.push(*node_1);
        }
    }
    adjacency_matrix
}

/// Double-Degree Array
///
/// Given: A simple graph with n≤10^3 vertices in the edge list format.
///
/// Return: An array D[1..n] where D[i] is the sum of the degrees of i's neighbors.
pub fn rosalind_ddeg() {
    let (num_nodes, _, edges) = utils::read_edge_list(&utils::input_from_file(
        "data/algorithmic_heights/rosalind_ddeg.txt",
    ));
    let degrees = get_degrees(&edges);
    let adjacency_matrix = make_adjacency_matrix(&edges, false);
    for node in 1..=num_nodes {
        match adjacency_matrix.get(&node) {
            Some(edge_list) => print!(
                "{} ",
                edge_list
                    .iter()
                    .map(|n| degrees.get(&n).unwrap_or(&0))
                    .sum::<usize>()
            ),
            None => print!("0 "),
        }
    }
}
