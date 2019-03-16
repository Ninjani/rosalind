use crate::utils;
use failure::Error;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::iter::repeat;

/// Dijkstra's Algorithm
///
/// Given: A simple directed graph with positive edge weights from 1 to 10^3 and n≤10^3 vertices in the edge list format.
///
/// Return: An array D[1..n] where D[i] is the length of a shortest path from the vertex 1 to the vertex i (D[1]=0). If i is not reachable from 1 set D[i] to −1.
pub fn rosalind_dij() -> Result<(), Error> {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_dij.txt");
    let mut lines = contents
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let (num_nodes, _, edges) = utils::read_weighted_edge_list(&mut lines)?;
    let adjacency_matrix = make_weighted_adjacency_matrix(&edges);
    for node in 1..=num_nodes {
        match dijkstra(num_nodes, &adjacency_matrix, 1, node) {
            Some(cost) => print!("{} ", cost),
            None => print!("-1 "),
        }
    }
    Ok(())
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

/// Makes weighted adjacency matrix from list of weighted edges
pub fn make_weighted_adjacency_matrix<
    T: Eq + Copy + ::std::hash::Hash,
    U: Eq + Copy + ::std::hash::Hash,
>(
    edges: &[(T, T, U)],
) -> HashMap<T, Vec<(T, U)>> {
    let mut adjacency_matrix = HashMap::new();
    for (node_1, node_2, weight) in edges {
        {
            let edge_list_1 = adjacency_matrix.entry(*node_1).or_insert_with(Vec::new);
            edge_list_1.push((*node_2, *weight));
        }
    }
    adjacency_matrix
}

/// Finds length of shortest (weighted) path from start_node to end_node using Dijkstra's Algorithm
pub fn dijkstra<S: ::std::hash::BuildHasher>(
    num_nodes: usize,
    adjacency_matrix: &HashMap<usize, Vec<(usize, isize)>, S>,
    start_node: usize,
    end_node: usize,
) -> Option<usize> {
    let mut distances = repeat(::std::usize::MAX)
        .take(num_nodes)
        .collect::<Vec<_>>();
    distances[start_node - 1] = 0;
    let mut heap = BinaryHeap::with_capacity(num_nodes);
    heap.push(State {
        cost: distances[start_node - 1],
        node: start_node,
    });
    while let Some(State { cost, node }) = heap.pop() {
        if node == end_node {
            return Some(cost);
        }
        if cost > distances[node - 1] {
            continue;
        }
        if let Some(edge_list) = adjacency_matrix.get(&node) {
            for (child, weight) in edge_list {
                let next = State {
                    cost: cost + (*weight as usize),
                    node: *child,
                };
                if next.cost < distances[next.node - 1] {
                    distances[next.node - 1] = next.cost;
                    heap.push(next);
                }
            }
        }
    }
    None
}
