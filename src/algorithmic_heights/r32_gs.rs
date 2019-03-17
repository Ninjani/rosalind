use crate::algorithmic_heights::{DFS, r5_ddeg::make_adjacency_matrix};
use crate::utils;
use failure::Error;

/// W.I.P
pub fn rosalind_gs() -> Result<(), Error> {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_gs.txt");
    let mut lines = contents
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let num_sections = lines.next().unwrap().parse::<usize>()?;
    for _ in 0..num_sections {
        let (num_nodes, _, edges) = utils::read_edge_list(&mut lines);
        let adjacency_matrix = make_adjacency_matrix(&edges, true);
        let node_order = DFS::get_sink_scc_node_order(&adjacency_matrix, num_nodes);
        let dfs_scc = DFS::run_dfs_given_node_order(adjacency_matrix, num_nodes, &node_order);

    }
    Ok(())
}
