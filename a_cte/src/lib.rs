use std::collections::BinaryHeap;

use failure::Error;

use a_dij::State;
use utility;
use utility::io::Parseable;

/// Shortest Cycle Through a Given Edge
///
/// Given: A positive integer k≤20 and k simple directed graphs with positive integer edge weights and at most 10^3 vertices in the edge list format.
///
/// Return: For each graph, output the length of a shortest cycle going through the first specified edge if there is a cycle and "-1" otherwise.
pub fn rosalind_cte(filename: &str) -> Result<Vec<isize>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mut sections = input.split('\n').filter(|line| !line.trim().is_empty());
    let num_graphs = sections.next().unwrap().parse::<usize>()?;
    let mut output = Vec::with_capacity(num_graphs);
    for _ in 0..num_graphs {
        let length_input = usize::parse_line(sections.next().unwrap())?;
        let (num_nodes, num_edges) = (length_input[0], length_input[1]);
        let mut graph = vec![format!("{} {}", num_nodes, num_edges)];
        graph.extend((0..num_edges).map(|_| sections.next().unwrap().to_owned()));
        let mut lines = graph.into_iter();
        let weighted_graph = utility::graph::WeightedGraph::from_weighted_edge_list(&mut lines)?;
        let (start, end, weight) = weighted_graph.edges[0];
        let min_distances =
            weighted_graph.get_dijkstra_start_to_all_distances(weighted_graph.node_to_index[&end]);
        match min_distances[weighted_graph.node_to_index[&start]] {
            Some(cost) => output.push(cost as isize + weight as isize),
            None => output.push(-1),
        }
    }
    println!("{}", utility::io::format_array(&output));
    Ok(output)
}

pub trait DijkstraStartToAll {
    fn get_dijkstra_start_to_all_distances(&self, start_node: usize) -> Vec<Option<usize>>;
}

impl DijkstraStartToAll for utility::graph::WeightedGraph {
    fn get_dijkstra_start_to_all_distances(&self, start_node: usize) -> Vec<Option<usize>> {
        let mut distances = (0..self.num_nodes).map(|_| None).collect::<Vec<_>>();
        let mut heap = BinaryHeap::with_capacity(self.num_nodes);
        heap.push(State {
            cost: 0,
            node: start_node,
        });
        while let Some(State { cost, node }) = heap.pop() {
            if distances[node].is_some() {
                continue;
            }
            distances[node] = Some(cost);
            if let Some(edge_list) = self.adjacency_list.get(&self.nodes[node]) {
                for (child, weight) in edge_list {
                    let next = State {
                        cost: cost + (*weight as usize),
                        node: self.node_to_index[child],
                    };
                    if distances[next.node].is_none() {
                        heap.push(next);
                    }
                }
            }
        }
        distances
    }
}

#[cfg(test)]
mod tests {
    use utility::io::Parseable;

    use super::*;

    #[test]
    fn cte() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_cte")?;
        let output = isize::parse_line(&utility::io::input_from_file(&output_file)?)?;
        assert_eq!(rosalind_cte(&input_file)?, output);
        Ok(())
    }
}
