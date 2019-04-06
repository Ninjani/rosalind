use crate::algorithmic_heights::{r5_ddeg::make_adjacency_matrix, DFS};
use crate::utils;
use failure::Error;
use hashbrown::HashMap;

pub fn rosalind_sdag() -> Result<(), Error> {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_sdag.txt");
    let mut lines = contents
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let (num_nodes, _, edges) = utils::read_weighted_edge_list(&mut lines)?;
    let raw_edges: Vec<_> = edges.iter().map(|(n1, n2, _)| (*n1, *n2)).collect();
    let edge_weights: HashMap<_, _> = edges.iter().map(|(n1, n2, w)| ((*n1, *n2), *w)).collect();
    let adjacency_matrix = make_adjacency_matrix(&raw_edges, true);
    let graph = DFS::run_dfs(adjacency_matrix, num_nodes);
    let distances = graph.get_shortest_path_length(&edge_weights, 1);
    utils::print_array(
        &distances
            .into_iter()
            .map(|d| match d {
                Some(distance) => distance.to_string(),
                None => "x".into(),
            })
            .collect::<Vec<_>>(),
    );
    Ok(())
}

impl DFS {
    pub fn get_shortest_path_length(
        &self,
        edge_weights: &HashMap<(usize, usize), isize>,
        source_node: usize,
    ) -> Vec<Option<isize>> {
        fn update(distances: &mut [isize], edge: &(usize, usize, isize)) -> bool {
            let (node_1, node_2, weight) = edge;
            if distances[*node_1 - 1] < ::std::isize::MAX
                && distances[*node_1 - 1] + weight < distances[*node_2 - 1]
            {
                distances[*node_2 - 1] = distances[*node_1 - 1] + weight;
                true
            } else {
                false
            }
        }
        let mut distances: Vec<_> = (0..self.num_nodes).map(|_| ::std::isize::MAX).collect();
        distances[source_node - 1] = 0;
        for node in DFS::get_topological_sort(&self) {
            if let Some(edges) = self.adjacency_matrix.get(&node) {
                for target in edges {
                    update(
                        &mut distances,
                        &(node, *target, edge_weights[&(node, *target)]),
                    );
                }
            }
        }
        distances
            .into_iter()
            .map(|d| if d < ::std::isize::MAX { Some(d) } else { None })
            .collect()
    }
}
