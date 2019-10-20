use std::collections::btree_map::BTreeMap;

use failure::Error;

use crate::utility;

pub fn rosalind_ba3g() -> Result<(), Error> {
    let graph = utility::graph::IntegerGraph::from_adjacency_list(
        &utility::io::input_from_file("data/textbook_track/rosalind_ba3g.txt")?,
        false,
    )?;
    println!(
        "{}",
        graph
            .get_eulerian_path()
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

impl utility::graph::IntegerGraph {
    pub fn get_eulerian_path(&self) -> Option<Vec<usize>> {
        let adjacency_list_reverse = reverse_adjacency_list(&self.adjacency_list);
        let (mut unbalanced_incoming, mut unbalanced_outgoing) = (None, None);
        for node in 0..self.num_nodes {
            let (incoming_count, outgoing_count) = (
                adjacency_list_reverse
                    .get(&self.nodes[node])
                    .unwrap_or(&Vec::new())
                    .len(),
                self.adjacency_list
                    .get(&self.nodes[node])
                    .unwrap_or(&Vec::new())
                    .len(),
            );
            if incoming_count > outgoing_count {
                unbalanced_incoming = Some(node);
            } else if outgoing_count > incoming_count {
                unbalanced_outgoing = Some(node);
            }
        }
        let mut new_graph = self.clone();
        let start_node = match (unbalanced_incoming, unbalanced_outgoing) {
            (Some(incoming), Some(outgoing)) => {
                new_graph
                    .adjacency_list
                    .entry(new_graph.nodes[incoming])
                    .or_insert_with(Vec::new)
                    .push(new_graph.nodes[outgoing]);
                Some(outgoing)
            }
            (None, None) => None,
            _ => return None,
        };
        match new_graph.get_eulerian_cycle(start_node) {
            Some(cycle) => {
                let length = cycle.len();
                Some(cycle.into_iter().take(length - 1).collect())
            }
            None => None,
        }
    }
}
