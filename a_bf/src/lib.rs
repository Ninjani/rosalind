use failure::Error;

use crate::utility;

/// Bellman-Ford Algorithm
///
/// Given: A simple directed graph with integer edge weights
/// from −10^3 to 10^3 and n≤10^3 vertices in the edge list format.
///
/// Return: An array D[1..n] where D[i] is the length of a shortest path
/// from the vertex 1 to the vertex i (D[1]=0). If i is not reachable from 1 set D[i] to x.
pub fn rosalind_bf(filename: &str) -> Result<Vec<Option<i32>>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mut lines = input.split('\n').map(|s| s.to_owned());
    let graph = utility::graph::WeightedGraph::from_weighted_edge_list(&mut lines)?;
    let distances = graph
        .bellman_ford(0)
        .ok_or_else(|| format_err!("Negative cycle found"))?;
    let mut output = Vec::with_capacity(graph.num_nodes);
    for node in 0..graph.num_nodes {
        if distances[node] < ::std::i32::MAX {
            output.push(Some(distances[node]));
            print!("{} ", distances[node]);
        } else {
            output.push(None);
            print!("x ");
        }
    }
    Ok(output)
}

/// Finds lengths of shortest weighted (incl. negative weights) paths from start_node to each other node
/// None if a negative weight cycle exists
impl utility::graph::WeightedGraph {
    pub fn bellman_ford(&self, start_index: usize) -> Option<Vec<i32>> {
        fn update(dists: &mut [i32], index_1: usize, index_2: usize, weight: i32) -> bool {
            if dists[index_1] < ::std::i32::MAX && dists[index_1] + weight < dists[index_2] {
                dists[index_2] = dists[index_1] + weight;
                true
            } else {
                false
            }
        }
        let mut distances = (0..self.num_nodes)
            .map(|_| ::std::i32::MAX)
            .collect::<Vec<_>>();
        distances[start_index] = 0;
        let mut updated;
        for _ in 0..self.num_nodes - 1 {
            updated = false;
            for (node_1, node_2, weight) in &self.edges {
                if update(
                    &mut distances,
                    self.node_to_index[node_1],
                    self.node_to_index[node_2],
                    *weight,
                ) {
                    updated = true;
                }
            }
            if !updated {
                break;
            }
        }
        for (node_1, node_2, weight) in &self.edges {
            if update(
                &mut distances,
                self.node_to_index[node_1],
                self.node_to_index[node_2],
                *weight,
            ) {
                return None;
            }
        }
        Some(distances)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bf() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_bf")?;
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
        assert_eq!(rosalind_bf(&input_file)?, output);
        Ok(())
    }
}
