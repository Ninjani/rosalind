use crate::textbook_track::r49_ba3f::get_eulerian_cycle;
use crate::utils;
use failure::Error;
use hashbrown::HashSet;
use std::collections::btree_map::BTreeMap;

pub fn rosalind_ba3g() -> Result<(), Error> {
    let (_, adjacency_list) = utils::read_adjacency_list(
        &utils::input_from_file("data/textbook_track/rosalind_ba3g.txt"),
        false,
    )?;
    println!(
        "{}",
        get_eulerian_path(adjacency_list)
            .unwrap()
            .into_iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join("->")
    );
    Ok(())
}

pub fn reverse_adjacency_list(
    adjacency_list: &BTreeMap<usize, Vec<usize>>,
) -> BTreeMap<usize, Vec<usize>> {
    let mut adjacency_list_reverse = BTreeMap::new();
    for (node_1, edges) in adjacency_list {
        for node_2 in edges {
            adjacency_list_reverse
                .entry(*node_2)
                .or_insert_with(Vec::new)
                .push(*node_1);
        }
    }
    adjacency_list_reverse
}

pub fn get_eulerian_path(adjacency_list: BTreeMap<usize, Vec<usize>>) -> Option<Vec<usize>> {
    let adjacency_list_reverse = reverse_adjacency_list(&adjacency_list);
    let (mut unbalanced_incoming, mut unbalanced_outgoing) = (None, None);
    let nodes: HashSet<_> = adjacency_list.keys().cloned().collect();
    for node in nodes.union(&adjacency_list_reverse.keys().cloned().collect()) {
        let (incoming_count, outgoing_count) = (
            adjacency_list_reverse
                .get(node)
                .unwrap_or(&Vec::new())
                .len(),
            adjacency_list.get(node).unwrap_or(&Vec::new()).len(),
        );
        if incoming_count > outgoing_count {
            unbalanced_incoming = Some(*node);
        } else if outgoing_count > incoming_count {
            unbalanced_outgoing = Some(*node);
        }
    }
    let mut adjacency_list = adjacency_list;
    let start_node = match (unbalanced_incoming, unbalanced_outgoing) {
        (Some(incoming), Some(outgoing)) => {
            adjacency_list
                .entry(incoming)
                .or_insert_with(Vec::new)
                .push(outgoing);
            Some(outgoing)
        }
        (None, None) => None,
        _ => return None,
    };
    match get_eulerian_cycle(adjacency_list, start_node, nodes.len()) {
        Some(cycle) => {
            let length = cycle.len();
            Some(cycle.into_iter().take(length - 1).collect())
        }
        None => None,
    }
}
