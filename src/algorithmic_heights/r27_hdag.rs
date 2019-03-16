use crate::algorithmic_heights::{r5_ddeg::make_adjacency_matrix, DFS};
use crate::utils;
use failure::Error;

/// Given: A positive integer k≤20 and k simple directed acyclic graphs
/// in the edge list format with at most 103 vertices each.
///
/// Return: For each graph, if it contains a Hamiltonian path output "1"
/// followed by a Hamiltonian path (i.e., a list of vertices), otherwise output "-1".
pub fn rosalind_hdag() -> Result<(), Error> {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_hdag.txt");
    let mut sections = contents.split("\n\n");
    sections.next().unwrap();
    for section in sections {
        let (num_nodes, _, edges) = utils::read_edge_list(section);
        let adjacency_matrix = make_adjacency_matrix(&edges, true);
        let hamiltonian_path = DFS::run_dfs(adjacency_matrix, num_nodes).get_hamiltonian_path();
        match hamiltonian_path {
            None => println!("-1"),
            Some(topo_sort) => println!(
                "1 {}",
                topo_sort
                    .into_iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
        }
    }
    Ok(())
}

impl DFS {
    pub fn get_hamiltonian_path(&self) -> Option<Vec<usize>> {
        if !self.is_acyclic() {
            None
        } else {
            let topo_sort = self.get_topological_sort();
            for i in 0..topo_sort.len() - 1 {
                match self.adjacency_matrix.get(&topo_sort[i]) {
                    Some(edge_list) => {
                        if !edge_list.contains(&topo_sort[i + 1]) {
                            return None;
                        }
                    }
                    None => return None,
                }
            }
            Some(topo_sort)
        }
    }
}
