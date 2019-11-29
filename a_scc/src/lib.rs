use failure::Error;

use utility;

/// Strongly Connected Components
///
/// Given: A simple directed graph with n≤103 vertices in the edge list format.
///
/// Return: The number of strongly connected components in the graph.
pub fn rosalind_scc(filename: &str) -> Result<usize, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mut lines = input
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let mut graph = utility::graph::IntegerGraph::from_edge_list(&mut lines, true, false)?;
    let graph_reverse = graph.get_reverse_graph(true);
    let mut node_order = graph_reverse
        .postvisit
        .into_iter()
        .enumerate()
        .collect::<Vec<_>>();
    node_order.sort_by(|a, b| b.1.cmp(&a.1));
    graph.run_dfs_given_node_order(&node_order.into_iter().map(|(i, _)| i).collect::<Vec<_>>());
    println!("{}", graph.num_connected_components);
    Ok(graph.num_connected_components)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scc() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_scc")?;
        let output = utility::io::input_from_file(&output_file)?.parse::<usize>()?;
        assert_eq!(rosalind_scc(&input_file)?, output);
        Ok(())
    }
}
