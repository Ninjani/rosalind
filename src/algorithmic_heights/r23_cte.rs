use crate::algorithmic_heights::r17_dij::{make_weighted_adjacency_matrix, State};
use crate::utils;
use crate::utils::Parseable;
use failure::Error;
use std::collections::{BinaryHeap, HashMap};
use std::iter::repeat;

/// Shortest Cycle Through a Given Edge
///
/// Given: A positive integer k≤20 and k simple directed graphs with positive integer edge weights and at most 10^3 vertices in the edge list format.
///
/// Return: For each graph, output the length of a shortest cycle going through the first specified edge if there is a cycle and "-1" otherwise.
pub fn rosalind_cte() -> Result<(), Error> {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_cte.txt");
    let mut sections = contents.split('\n');
    let num_graphs = sections.next().unwrap().parse::<usize>()?;
    for _ in 0..num_graphs {
        let length_input = usize::parse_line(sections.next().unwrap())?;
        let (num_nodes, num_edges) = (length_input[0], length_input[1]);
        let mut graph = vec![format!("{} {}", num_nodes, num_edges)];
        graph.extend((0..num_edges).map(|_| sections.next().unwrap().to_owned()));
        let (num_nodes, _, edges) = utils::read_weighted_edge_list(&graph.join("\n"))?;
        let adjacency_matrix = make_weighted_adjacency_matrix(&edges);
        let (start, end, weight) = edges[0];
        let min_distances = dijkstra_min_distances(num_nodes, &adjacency_matrix, end);
        match min_distances[start - 1] {
            Some(cost) => print!("{} ", cost + weight as usize),
            None => print!("-1 "),
        }
    }
    Ok(())
}

fn dijkstra_min_distances(
    num_nodes: usize,
    adjacency_matrix: &HashMap<usize, Vec<(usize, isize)>>,
    start_node: usize,
) -> Vec<Option<usize>> {
    let mut distances = repeat(None).take(num_nodes).collect::<Vec<_>>();
    let mut heap = BinaryHeap::with_capacity(num_nodes);
    heap.push(State {
        cost: 0,
        node: start_node,
    });
    while let Some(State { cost, node }) = heap.pop() {
        if distances[node - 1].is_some() {
            continue;
        }
        distances[node - 1] = Some(cost);
        if let Some(edge_list) = adjacency_matrix.get(&node) {
            for (child, weight) in edge_list {
                let next = State {
                    cost: cost + (*weight as usize),
                    node: *child,
                };
                if distances[next.node - 1].is_none() {
                    heap.push(next);
                }
            }
        }
    }
    distances
}
