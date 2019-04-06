use crate::textbook_track::r49_ba3f::get_eulerian_cycle;
use crate::utils;
use failure::Error;
use hashbrown::HashMap;
use petgraph::stable_graph::StableGraph;
use petgraph::Direction::{Incoming, Outgoing};
use std::fmt::Debug;
use std::hash::Hash;

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

pub fn get_eulerian_path<T: Hash + Clone + Eq + Debug>(
    adjacency_list: HashMap<T, Vec<T>>,
) -> Option<Vec<T>> {
    let mut graph = StableGraph::new();
    let mut node_to_index = HashMap::new();
    for (node_1, edges) in &adjacency_list {
        let index_1 = *node_to_index
            .entry(node_1.clone())
            .or_insert_with(|| graph.add_node(node_1.clone()));
        for node_2 in edges {
            let index_2 = *node_to_index
                .entry(node_2.clone())
                .or_insert_with(|| graph.add_node(node_2.clone()));
            graph.add_edge(index_1, index_2, ());
        }
    }
    let (mut unbalanced_incoming, mut unbalanced_outgoing) = (None, None);
    for (node, index) in node_to_index {
        let (incoming_count, outgoing_count) = (
            graph.edges_directed(index, Incoming).count(),
            graph.edges_directed(index, Outgoing).count(),
        );
        if incoming_count > outgoing_count {
            unbalanced_incoming = Some(node);
        } else if outgoing_count > incoming_count {
            unbalanced_outgoing = Some(node);
        }
    }
    let mut adjacency_list = adjacency_list;
    match (unbalanced_incoming, unbalanced_outgoing) {
        (Some(incoming), Some(outgoing)) => {
            adjacency_list
                .entry(incoming.clone())
                .or_insert_with(Vec::new)
                .push(outgoing.clone());
            match get_eulerian_cycle(&mut adjacency_list, outgoing) {
                Some(cycle) => {
                    let length = cycle.len();
                    Some(cycle.into_iter().take(length - 1).collect())
                }
                None => None,
            }
        }
        _ => None,
    }
}
