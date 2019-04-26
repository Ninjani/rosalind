use crate::algorithmic_heights::r5_ddeg::make_adjacency_list;
use crate::utils;
use failure::Error;
use std::collections::VecDeque;
use std::iter::repeat;
use std::collections::btree_map::BTreeMap;

/// Testing Bipartiteness
///
/// Given: A positive integer k≤20 and k simple graphs in the edge list format with at most 10^3 vertices each.
///
/// Return: For each graph, output "1" if it is bipartite and "-1" otherwise.
pub fn rosalind_bip() -> Result<(), Error> {
    let contents = utils::input_from_file("data/algorithmic_heights/rosalind_bip.txt");
    let mut lines = contents
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_owned());
    let num_sections = lines.next().unwrap().parse::<usize>()?;
    for _ in 0..num_sections {
        let (num_nodes, _, edges) = utils::read_edge_list(&mut lines, true);
        let adjacency_matrix = make_adjacency_list(&edges, false);
        if is_bipartite(num_nodes, &adjacency_matrix) {
            print!("1 ");
        } else {
            print!("-1 ");
        }
    }
    Ok(())
}

fn is_bipartite(num_nodes: usize, adjacency_matrix: &BTreeMap<usize, Vec<usize>>) -> bool {
    let mut colors = repeat(None).take(num_nodes).collect::<Vec<_>>();
    for node in 0..num_nodes {
        if colors[node].is_none() && !is_bipartite_checker(&mut colors, node, &adjacency_matrix)
        {
            return false;
        }
    }
    true
}

fn is_bipartite_checker(
    colors: &mut [Option<bool>],
    node: usize,
    adjacency_matrix: &BTreeMap<usize, Vec<usize>>,
) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back(node);
    colors[node] = Some(true);
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        if let Some(edge_list) = adjacency_matrix.get(&node) {
            for &child in edge_list {
                if child == node {
                    return false;
                }
                if colors[child].is_none() {
                    colors[child] = Some(!colors[node].unwrap());
                    queue.push_back(child);
                } else if colors[node] == colors[child] {
                    return false;
                }
            }
        }
    }
    true
}
