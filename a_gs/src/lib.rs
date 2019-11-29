use failure::Error;

use utility;

/// W.I.P
pub fn rosalind_gs() -> Result<(), Error> {
    let contents = utility::io::input_from_file("data/algorithmic_heights/rosalind_gs.txt")?;
    let mut lines = contents
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let num_sections = lines.next().unwrap().parse::<usize>()?;
    for _ in 0..num_sections {
        let mut graph = utility::graph::IntegerGraph::from_edge_list(&mut lines, true, true)?;
        let graph_reverse = graph.get_reverse_graph(true);
        let mut node_order = graph_reverse
            .postvisit
            .into_iter()
            .enumerate()
            .collect::<Vec<_>>();
        node_order.sort_by(|a, b| b.1.cmp(&a.1));
        graph.run_dfs_given_node_order(&node_order.into_iter().map(|(i, _)| i).collect::<Vec<_>>());
    }
    Ok(())
}
