use crate::algorithmic_heights::r5_ddeg::make_adjacency_matrix;
use crate::utils;
use std::collections::{HashMap, HashSet};

/// Square in a Graph
///
/// Given: A positive integer k≤20 and k simple undirected graphs with n≤400 vertices in the edge list format.
///
/// Return: For each graph, output "1" if it contains a simple cycle (that is, a cycle which doesn’t intersect itself) of length 4 and "-1" otherwise.
pub fn rosalind_sq() {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_sq.txt");
    let mut sections = contents.split("\n\n");
    sections.next().unwrap();
    for section in sections {
        let (num_nodes, _, edges) = utils::read_edge_list(section);
        let adjacency_matrix = make_adjacency_matrix(&edges, false);
        if has_square(num_nodes, &adjacency_matrix) {
            print!("1 ")
        } else {
            print!("-1 ")
        }
    }
}

fn has_square(num_nodes: usize, adjacency_matrix: &HashMap<usize, Vec<usize>>) -> bool {
    for i in 1..num_nodes {
        for j in (i + 1)..=num_nodes {
            let adj_i: HashSet<_> = adjacency_matrix
                .get(&i)
                .unwrap_or(&vec![])
                .iter()
                .cloned()
                .collect();
            let adj_j: HashSet<_> = adjacency_matrix
                .get(&j)
                .unwrap_or(&vec![])
                .iter()
                .cloned()
                .collect();
            if adj_i.intersection(&adj_j).collect::<HashSet<_>>().len() > 1 {
                return true;
            }
        }
    }
    false
}
