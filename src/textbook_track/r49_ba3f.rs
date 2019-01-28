use crate::utils;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use failure::Error;

pub fn rosalind_ba3f() -> Result<(), Error> {
    let mut adjacency_list = utils::read_adjacency_list(&utils::input_from_file(
        "data/textbook_track/rosalind_ba3f.txt",
    ))?;
    let cycle = get_eulerian_cycle(&mut adjacency_list);
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
) -> Vec<T> {
    let nodes: Vec<_> = adjacency_list.keys().cloned().collect();
    let node_to_index: HashMap<_, _> = nodes.iter().enumerate().map(|(i, n)| (n, i)).collect();
    let mut num_edges_per_node: Vec<_> = nodes
        .iter()
        .map(|n| adjacency_list.get(n).unwrap_or(&Vec::new()).len())
        .collect();
    let mut current_cycle = Vec::new();
    let mut final_cycle = Vec::new();
    let mut current_node = nodes[0].clone();
    let mut next_node;
    current_cycle.push(current_node.clone());
    while !current_cycle.is_empty() {
        println!("{:?} {:?}", node_to_index, current_node);
        if num_edges_per_node[node_to_index[&current_node]] > 0 {
            current_cycle.push(current_node.clone());
            next_node = adjacency_list
                .entry(current_node.clone())
                .or_insert_with(Vec::new)
                .pop()
                .unwrap();
            num_edges_per_node[node_to_index[&current_node]] -= 1;
            current_node = next_node;
        } else {
            final_cycle.push(current_node.clone());
            current_node = current_cycle.pop().unwrap();
        }
    }
    final_cycle.into_iter().rev().collect()
}
