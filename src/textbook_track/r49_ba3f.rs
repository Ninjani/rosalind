use crate::utils;
use failure::Error;
use hashbrown::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub fn rosalind_ba3f() -> Result<(), Error> {
    let (_, mut adjacency_list) = utils::read_adjacency_list(
        &utils::input_from_file("data/textbook_track/rosalind_ba3f.txt"),
        false,
    )?;
    let cycle = get_eulerian_cycle(&mut adjacency_list, 0).unwrap();
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

pub fn get_eulerian_cycle<T: Hash + Clone + Eq + Debug>(
    adjacency_list: &mut HashMap<T, Vec<T>>,
    start_node: T,
) -> Option<Vec<T>> {
    let nodes: Vec<_> = adjacency_list.keys().cloned().collect();
    let node_to_index: HashMap<_, _> = nodes.iter().enumerate().map(|(i, n)| (n, i)).collect();
    let mut num_edges_per_node: Vec<_> = nodes
        .iter()
        .map(|n| adjacency_list.get(n).unwrap_or(&Vec::new()).len())
        .collect();
    let mut current_cycle = Vec::new();
    let mut final_cycle = Vec::new();
    let mut current_node = start_node;
    let mut next_node;
    current_cycle.push(current_node.clone());
    let mut start = false;
    while !start || !current_cycle.is_empty() {
        start = true;
        match node_to_index.get(&current_node) {
            Some(index) => {
                if num_edges_per_node[*index] > 0 {
                    current_cycle.push(current_node.clone());
                    next_node = adjacency_list
                        .entry(current_node.clone())
                        .or_insert_with(Vec::new)
                        .pop()
                        .unwrap();
                    num_edges_per_node[*index] -= 1;
                    current_node = next_node;
                } else {
                    final_cycle.push(current_node.clone());
                    current_node = current_cycle.pop().unwrap();
                }
            }
            None => return None,
        }
    }
    Some(final_cycle.into_iter().rev().collect())
}
