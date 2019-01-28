use crate::algorithmic_heights::r10_cc::count_connected_components;
use crate::utils;
use failure::Error;

/// Completing a Tree
///
/// Given: A positive integer n (n≤1000) and an adjacency list corresponding to a graph on n nodes that contains no cycles.
///
/// Return: The minimum number of edges that can be added to the graph to produce a tree.
pub fn rosalind_tree() -> Result<(), Error> {
    let (num_nodes, adjacency_matrix) = utils::read_adjacency_matrix(
        &utils::input_from_file("data/stronghold/rosalind_tree.txt"),
        false,
    )?;
    println!(
        "{}",
        count_connected_components(num_nodes, &adjacency_matrix) - 1
    );
    Ok(())
}
