use failure::Error;

use a_ts::TopologicalSort;
use utility;

/// Find a Topological Ordering of a DAG
///
/// Given: The adjacency list of a graph (with nodes represented by integers).
///
/// Return: A topological ordering of this graph.
pub fn rosalind_ba5n(filename: &str) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let graph = utility::graph::IntegerGraph::from_adjacency_list(&contents, true)?;
    println!(
        "{}",
        graph
            .get_topological_sort()
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
    Ok(())
}
