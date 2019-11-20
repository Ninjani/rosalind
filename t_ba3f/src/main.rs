use std::collections::HashSet;

use failure::Error;

use crate::utility;

pub fn rosalind_ba3f() -> Result<(), Error> {
    let graph = utility::graph::IntegerGraph::from_adjacency_list(
        &utility::io::input_from_file("data/textbook_track/rosalind_ba3f.txt")?,
        false,
    )?;
    let cycle = graph.get_eulerian_cycle(None).unwrap();
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

impl utility::graph::IntegerGraph {
    pub fn get_eulerian_cycle(&self, start_node: Option<usize>) -> Option<Vec<usize>> {
        let mut adjacency_list = self.adjacency_list.clone();
        let mut num_edges_per_node: Vec<_> = self
            .nodes
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
        let mut final_nodes = HashSet::with_capacity(self.num_nodes);
        while !current_cycle.is_empty() {
            if num_edges_per_node[current_node_index] > 0 {
                current_cycle.push(current_node_index);
                next_node_index = self.node_to_index[&adjacency_list
                    .entry(self.nodes[current_node_index])
                    .or_insert_with(Vec::new)
                    .pop()
                    .unwrap()];
                num_edges_per_node[current_node_index] -= 1;
                current_node_index = next_node_index;
            } else {
                final_cycle.push(current_node_index);
                final_nodes.insert(current_node_index);
                current_node_index = current_cycle.pop().unwrap();
            }
        }
        let length = final_cycle.len();
        if final_nodes.len() == self.num_nodes && final_cycle[0] == final_cycle[length - 1] {
            Some(
                final_cycle
                    .into_iter()
                    .rev()
                    .map(|n| self.nodes[n])
                    .collect(),
            )
        } else {
            None
        }
    }
}
