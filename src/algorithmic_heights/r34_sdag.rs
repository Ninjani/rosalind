use failure::Error;

use crate::utility;

/// Shortest Paths in DAG
///
/// Given: A weighted DAG with integer edge weights from −103 to 103
/// and n≤105 vertices in the edge list format.
///
/// Return: An array D[1..n] where D[i] is the length of a shortest path
/// from the vertex 1 to the vertex i (D[1]=0).
/// If i is not reachable from 1 set D[i] to x.
pub fn rosalind_sdag(filename: &str) -> Result<Vec<Option<i32>>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mut lines = input
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let weighted_graph = utility::graph::WeightedGraph::from_weighted_edge_list(&mut lines)?;
    let graph = utility::graph::IntegerGraph::from_weighted_graph(&weighted_graph, true);
    let topo_sort = graph.get_topological_sort();
    let distances = weighted_graph.get_shortest_path_length(0, &topo_sort);
    println!(
        "{}",
        utility::io::format_array(
            &distances
                .iter()
                .map(|d| match d {
                    Some(distance) => distance.to_string(),
                    None => "x".into(),
                })
                .collect::<Vec<_>>(),
        )
    );
    Ok(distances)
}

impl utility::graph::WeightedGraph {
    pub fn get_shortest_path_length(
        &self,
        source_node: usize,
        topo_sort: &[usize],
    ) -> Vec<Option<i32>> {
        fn update(distances: &mut [i32], node_1: usize, node_2: usize, weight: i32) -> bool {
            if distances[node_1] < ::std::i32::MAX && distances[node_1] + weight < distances[node_2]
            {
                distances[node_2] = distances[node_1] + weight;
                true
            } else {
                false
            }
        }
        let mut distances: Vec<_> = (0..self.num_nodes).map(|_| ::std::i32::MAX).collect();
        distances[source_node] = 0;
        for node_1 in topo_sort {
            if let Some(edge_list) = self.adjacency_list.get(node_1) {
                for (node_2, weight) in edge_list {
                    update(
                        &mut distances,
                        self.node_to_index[node_1],
                        self.node_to_index[node_2],
                        *weight,
                    );
                }
            }
        }
        distances
            .into_iter()
            .map(|d| if d < ::std::i32::MAX { Some(d) } else { None })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sdag() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_sdag")?;
        let output: Vec<_> = utility::io::input_from_file(&output_file)?
            .split_whitespace()
            .map(|line| {
                if line == "x" {
                    None
                } else {
                    Some(line.parse::<i32>().unwrap())
                }
            })
            .collect();
        assert_eq!(rosalind_sdag(&input_file)?, output);
        Ok(())
    }
}
