use crate::utils;
use failure::Error;
use hashbrown::HashSet;
use std::collections::btree_map::BTreeMap;

pub fn rosalind_ba3f() -> Result<(), Error> {
    let (num_nodes, adjacency_list) = utils::read_adjacency_list(
        &utils::input_from_file("data/textbook_track/rosalind_ba3f.txt"),
        false,
    )?;
    let cycle = get_eulerian_cycle(adjacency_list, None, num_nodes).unwrap();
    println!(
        "{}",
        cycle
            .into_iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join("->")
    );
    Ok(())
}

pub fn get_eulerian_cycle(
    adjacency_list: BTreeMap<usize, Vec<usize>>,
    start_node: Option<usize>,
    num_nodes: usize
) -> Option<Vec<usize>> {
    let mut adjacency_list = adjacency_list;
    let nodes: Vec<_> = (0..num_nodes).collect();
    let mut num_edges_per_node: Vec<_> = nodes
        .iter()
        .map(|n| adjacency_list.get(n).unwrap_or(&Vec::new()).len())
        .collect();
    let mut current_cycle = Vec::new();
    let mut final_cycle = Vec::new();
    let mut current_node_index = match start_node {
        Some(start_node) => start_node,
        None => 0,
    };
    let mut next_node_index;
    current_cycle.push(current_node_index);
    let mut final_nodes = HashSet::with_capacity(num_nodes);
    while !current_cycle.is_empty() {
        if num_edges_per_node[current_node_index] > 0 {
            current_cycle.push(current_node_index);
            next_node_index = adjacency_list
                    .entry(current_node_index)
                    .or_insert_with(Vec::new)
                    .pop()
                    .unwrap();
            num_edges_per_node[current_node_index] -= 1;
            current_node_index = next_node_index;
        } else {
            final_cycle.push(current_node_index);
            final_nodes.insert(current_node_index);
            current_node_index = current_cycle.pop().unwrap();
        }
    }
    if final_nodes.len() == num_nodes {
        Some(final_cycle.into_iter().rev().collect())
    } else {
        None
    }
}
