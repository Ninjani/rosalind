use crate::algorithmic_heights::{r5_ddeg::make_adjacency_matrix, DFS};
use crate::utils;

/// Testing Acyclicity
///
/// Given: A positive integer k≤20 and k simple directed graphs in the edge list format with at most 10^3 vertices and 3⋅10^3 edges each.
///
/// Return: For each graph, output "1" if the graph is acyclic and "-1" otherwise.
pub fn rosalind_dag() {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_dag.txt");
    let mut sections = contents.split("\n\n");
    sections.next().unwrap();
    for section in sections {
        let (num_nodes, _, edges) = utils::read_edge_list(section);
        let adjacency_matrix = make_adjacency_matrix(&edges, true);
        let dfs = DFS::run_dfs(adjacency_matrix, num_nodes);
        if dfs.is_acyclic() {
            print!("1 ")
        } else {
            print!("-1 ")
        }
    }
}

impl DFS {
    pub fn is_acyclic(&self) -> bool {
        for node in 1..self.num_nodes + 1 {
            if let Some(edge_list) = self.adjacency_matrix.get(&node) {
                for next_node in edge_list {
                    if self.previsit[*next_node - 1] < self.previsit[node - 1]
                        && self.previsit[node - 1] < self.postvisit[node - 1]
                        && self.postvisit[node - 1] < self.postvisit[*next_node - 1]
                    {
                        return false;
                    }
                }
            }
        }
        true
    }
}
