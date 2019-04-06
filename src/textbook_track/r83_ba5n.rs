use crate::algorithmic_heights::DFS;
use crate::utils;
use failure::Error;

/// Find a Topological Ordering of a DAG
///
/// Given: The adjacency list of a graph (with nodes represented by integers).
///
/// Return: A topological ordering of this graph.
pub fn rosalind_ba5n() -> Result<(), Error> {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba5n.txt");
    let (num_nodes, adjacency_matrix) = utils::read_adjacency_list(&contents, true)?;
    println!(
        "{}",
        DFS::run_dfs(adjacency_matrix, num_nodes)
            .get_topological_sort()
            .iter()
            .map(|n| (n - 1).to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
    Ok(())
}
