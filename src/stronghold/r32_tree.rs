use crate::algorithmic_heights::DFS;
use crate::utils;
use failure::Error;

/// Completing a Tree
///
/// Given: A positive integer n (n≤1000) and an adjacency list corresponding to a graph on n nodes that contains no cycles.
///
/// Return: The minimum number of edges that can be added to the graph to produce a tree.
pub fn rosalind_tree() -> Result<(), Error> {
    let (num_nodes, adjacency_list) = utils::read_edge_list_to_adjacency_list(
        &utils::input_from_file("data/stronghold/rosalind_tree.txt"),
        false,
        true
    )?;
    println!(
        "{}",
        DFS::run_dfs(adjacency_list, num_nodes).num_connected_components - 1
    );
    Ok(())
}
