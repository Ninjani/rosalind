use crate::algorithmic_heights::r5_ddeg::make_adjacency_list;
use crate::utils;
use failure::Error;
use hashbrown::{HashSet};
use std::collections::btree_map::BTreeMap;

/// Square in a Graph
///
/// Given: A positive integer k≤20 and k simple undirected graphs with n≤400 vertices in the edge list format.
///
/// Return: For each graph, output "1" if it contains a simple cycle (that is, a cycle which doesn’t intersect itself) of length 4 and "-1" otherwise.
pub fn rosalind_sq() -> Result<(), Error> {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_sq.txt");
    let mut lines = contents
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let num_sections = lines.next().unwrap().parse::<usize>()?;
    for _ in 0..num_sections {
        let (num_nodes, _, edges) = utils::read_edge_list(&mut lines, true);
        let adjacency_matrix = make_adjacency_list(&edges, false);
        if has_square(num_nodes, &adjacency_matrix) {
            print!("1 ")
        } else {
            print!("-1 ")
        }
    }
    Ok(())
}

fn has_square(num_nodes: usize, adjacency_matrix: &BTreeMap<usize, Vec<usize>>) -> bool {
    for i in 0..num_nodes-1 {
        for j in (i+1)..num_nodes {
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
