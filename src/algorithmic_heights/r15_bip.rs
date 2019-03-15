use crate::algorithmic_heights::r5_ddeg::make_adjacency_matrix;
use crate::utils;
use std::collections::VecDeque;
use hashbrown::HashMap;
use std::iter::repeat;

/// Testing Bipartiteness
///
/// Given: A positive integer k≤20 and k simple graphs in the edge list format with at most 10^3 vertices each.
///
/// Return: For each graph, output "1" if it is bipartite and "-1" otherwise.
pub fn rosalind_bip() {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_bip.txt");
    let mut sections = contents.split("\n\n");
    sections.next().unwrap();
    for section in sections {
        let (num_nodes, _, edges) = utils::read_edge_list(section);
        let adjacency_matrix = make_adjacency_matrix(&edges, false);
        if is_bipartite(num_nodes, &adjacency_matrix) {
            print!("1 ");
        } else {
            print!("-1 ");
        }
    }
}

fn is_bipartite(num_nodes: usize, adjacency_matrix: &HashMap<usize, Vec<usize>>) -> bool {
    let mut colors = repeat(None).take(num_nodes).collect::<Vec<_>>();
    for node in 1..=num_nodes {
        if colors[node - 1].is_none() && !is_bipartite_checker(&mut colors, node, &adjacency_matrix)
        {
            return false;
        }
    }
    true
}

fn is_bipartite_checker<S: ::std::hash::BuildHasher>(
    colors: &mut [Option<bool>],
    node: usize,
    adjacency_matrix: &HashMap<usize, Vec<usize>, S>,
) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back(node);
    colors[node - 1] = Some(true);
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        if let Some(edge_list) = adjacency_matrix.get(&node) {
            for child in edge_list {
                if *child == node {
                    return false;
                }
                if colors[*child - 1].is_none() {
                    colors[*child - 1] = Some(!colors[node - 1].unwrap());
                    queue.push_back(*child);
                } else if colors[node - 1] == colors[*child - 1] {
                    return false;
                }
            }
        }
    }
    true
}
