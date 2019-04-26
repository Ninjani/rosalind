use crate::algorithmic_heights::{r5_ddeg::make_adjacency_list, DFS};
use crate::utils;
use failure::Error;

/// Testing Acyclicity
///
/// Given: A positive integer k≤20 and k simple directed graphs in the edge list format with at most 10^3 vertices and 3⋅10^3 edges each.
///
/// Return: For each graph, output "1" if the graph is acyclic and "-1" otherwise.
pub fn rosalind_dag() -> Result<(), Error> {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_dag.txt");
    let mut lines = contents
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let num_sections = lines.next().unwrap().parse::<usize>()?;
    for _ in 0..num_sections {
        let (num_nodes, _, edges) = utils::read_edge_list(&mut lines, true);
        let adjacency_matrix = make_adjacency_list(&edges, true);
        let dfs = DFS::run_dfs(adjacency_matrix, num_nodes);
        if dfs.is_acyclic() {
            print!("1 ")
        } else {
            print!("-1 ")
        }
    }
    Ok(())
}

impl DFS {
    pub fn is_acyclic(&self) -> bool {
        for node in 0..self.num_nodes {
            if let Some(edge_list) = self.adjacency_list.get(&node) {
                for &next_node in edge_list {
                    if self.previsit[next_node] < self.previsit[node]
                        && self.previsit[node] < self.postvisit[node]
                        && self.postvisit[node] < self.postvisit[next_node]
                    {
                        return false;
                    }
                }
            }
        }
        true
    }
}
