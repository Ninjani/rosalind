use crate::utils;
use std::iter::repeat;

const MAX_WEIGHT: isize = 1001;

/// Bellman-Ford Algorithm
///
/// Given: A simple directed graph with integer edge weights from −10^3 to 10^3 and n≤10^3 vertices in the edge list format.
///
/// Return: An array D[1..n] where D[i] is the length of a shortest path from the vertex 1 to the vertex i (D[1]=0). If i is not reachable from 1 set D[i] to x.
pub fn rosalind_bf() {
    let (num_nodes, _, edges) = utils::read_weighted_edge_list(&utils::input_from_file(
        "data/algorithmic_heights/rosalind_bf.txt",
    ));
    let distances = bellman_ford(num_nodes, &edges, 1);
    for node in 1..=num_nodes {
        match distances[node - 1] {
            Some(cost) => print!("{} ", cost),
            None => print!("x "),
        }
    }
}

/// Finds lengths of shortest weighted (incl. negative weights) paths from start_node to each other node
fn bellman_ford(
    num_nodes: usize,
    edges: &[(usize, usize, isize)],
    start_node: usize,
) -> Vec<Option<isize>> {
    let mut distances = repeat(None).take(num_nodes).collect::<Vec<_>>();
    distances[start_node - 1] = Some(0);
    for _ in 1..num_nodes {
        for edge in edges {
            let (node_1, node_2, weight) = edge;
            match (distances[*node_1 - 1], distances[*node_2 - 1]) {
                (Some(cost_1), Some(cost_2)) => {
                    distances[*node_2 - 1] = Some(cost_2.min(cost_1 + weight))
                }
                (Some(cost_1), None) => distances[*node_2 - 1] = Some(cost_1 + weight),
                (None, _) => (),
            }
        }
    }
    distances
}
