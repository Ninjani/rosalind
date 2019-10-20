use std::cmp::Ordering;
use std::collections::BinaryHeap;

use failure::Error;

use crate::utility;

/// Dijkstra's Algorithm
///
/// Given: A simple directed graph with positive edge weights from 1 to 10^3 and n≤10^3 vertices in the edge list format.
///
/// Return: An array D[1..n] where D[i] is the length of a shortest path from the vertex 1 to the vertex i (D[1]=0). If i is not reachable from 1 set D[i] to −1.
pub fn rosalind_dij(filename: &str) -> Result<Vec<isize>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mut lines = input
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let graph = utility::graph::WeightedGraph::from_weighted_edge_list(&mut lines)?;
    let mut lengths = Vec::with_capacity(graph.num_nodes);
    for node in 0..graph.num_nodes {
        match graph.dijkstra(0, node) {
            Some(cost) => lengths.push(cost as isize),
            None => lengths.push(-1),
        }
    }
    println!("{}", utility::io::format_array(&lengths));
    Ok(lengths)
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State {
    pub cost: usize,
    pub node: usize,
}

/// Makes State min-heapable
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Finds length of shortest (weighted) path from start_node to end_node using Dijkstra's Algorithm
impl utility::graph::WeightedGraph {
    pub fn dijkstra(&self, start_node: usize, end_node: usize) -> Option<usize> {
        let mut distances = (0..self.num_nodes)
            .map(|_| ::std::usize::MAX)
            .collect::<Vec<_>>();
        distances[start_node] = 0;
        let mut heap = BinaryHeap::with_capacity(self.num_nodes);
        heap.push(State {
            cost: distances[start_node],
            node: start_node,
        });
        while let Some(State { cost, node }) = heap.pop() {
            if node == end_node {
                return Some(cost);
            }
            if cost > distances[node] {
                continue;
            }
            if let Some(edge_list) = self.adjacency_list.get(&self.nodes[node]) {
                for (child, weight) in edge_list {
                    let next = State {
                        cost: cost + (*weight as usize),
                        node: self.node_to_index[child],
                    };
                    if next.cost < distances[next.node] {
                        distances[next.node] = next.cost;
                        heap.push(next);
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::utility::io::Parseable;

    use super::*;

    #[test]
    fn dij() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_dij")?;
        let output = isize::parse_line(&utility::io::input_from_file(&output_file)?)?;
        assert_eq!(rosalind_dij(&input_file)?, output);
        Ok(())
    }
}
