use crate::algorithmic_heights::{r5_ddeg::make_adjacency_list, DFS};
use crate::utils;
use failure::Error;
use itertools::Itertools;

/// W.I.P
pub fn rosalind_sc() -> Result<(), Error> {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_sc.txt");
    let mut lines = contents
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let num_sections = lines.next().unwrap().parse::<usize>()?;
    for _ in 0..num_sections {
        let (num_nodes, _, edges) = utils::read_edge_list(&mut lines, true);
        let adjacency_matrix = make_adjacency_list(&edges, true);
        let graph = DFS::run_dfs(adjacency_matrix, num_nodes);
        print!("{} ", if graph.is_semi_connected() { 1 } else { -1 });
    }
    Ok(())
}

impl DFS {
    pub fn is_semi_connected(&self) -> bool {
        for (node_1, node_2) in DFS::get_topological_sort(&self).into_iter().tuple_windows() {
            if let Some(edges) = self.adjacency_list.get(&node_1) {
                if !edges.contains(&node_2) {
                    return false;
                }
            }
        }
        true
    }
}
