use std::collections::{HashMap, HashSet};
use std::collections::btree_map::BTreeMap;

use failure::Error;

use crate::textbook_track::r50_ba3g::reverse_adjacency_list;
use crate::textbook_track::r55_ba3l::{
    get_string_spelled_by_gapped_patterns, PairedRead, read_paired_reads,
};
use crate::utility;

/// Reconstruct a String from its Paired Composition
///
/// Given: Integers k and d followed by a collection of paired k-mers PairedReads.
///
/// Return: A string Text with (k, d)-mer composition equal to PairedReads.
/// (If multiple answers exist, you may return any one.)
pub fn rosalind_ba3j() -> Result<(), Error> {
    let contents = utility::io::input_from_file("data/textbook_track/rosalind_ba3j.txt")?;
    let (paired_reads, k, d) = read_paired_reads(&contents);
    let adjacency_list = paired_de_bruijn_graph(&paired_reads);
    let (index_to_node, indexed_adjacency_list) = utility::graph::convert_graph(&adjacency_list);
    let graph = utility::graph::IntegerGraph::new(
        indexed_adjacency_list,
        (0..index_to_node.len()).collect(),
        false,
    );
    for cycle in graph.get_all_eulerian_paths() {
        println!(
            "{}",
            get_string_spelled_by_gapped_patterns(
                &cycle
                    .into_iter()
                    .map(|n| &index_to_node[&n])
                    .collect::<Vec<_>>(),
                k,
                d,
            )
                .unwrap()
        );
    }
    Ok(())
}

pub fn paired_de_bruijn_graph(nodes: &[PairedRead]) -> HashMap<PairedRead, Vec<PairedRead>> {
    fn prefix(pr: &PairedRead) -> PairedRead {
        (
            pr.0.chars().take(pr.0.len() - 1).collect(),
            pr.1.chars().take(pr.1.len() - 1).collect(),
        )
    }
    fn suffix(pr: &PairedRead) -> PairedRead {
        (
            pr.0.chars().skip(1).collect(),
            pr.1.chars().skip(1).collect(),
        )
    }
    let mut adjacency_list = HashMap::new();
    for node in nodes {
        adjacency_list
            .entry(prefix(node))
            .or_insert_with(Vec::new)
            .push(suffix(node));
    }
    adjacency_list
}

impl utility::graph::IntegerGraph {
    fn get_bypass_graph(&self, incoming_u: usize, node_v: usize, outgoing_w: usize) -> Self {
        let mut new_adj_list = BTreeMap::new();
        let new_node = self.num_nodes;
        for (node_1, edges) in &self.adjacency_list {
            for node_2 in edges {
                if *node_1 == incoming_u && *node_2 == node_v {
                    new_adj_list
                        .entry(*node_1)
                        .or_insert_with(Vec::new)
                        .push(new_node);
                } else if *node_1 == node_v && *node_2 == outgoing_w {
                    new_adj_list
                        .entry(new_node)
                        .or_insert_with(Vec::new)
                        .push(outgoing_w);
                } else {
                    new_adj_list
                        .entry(*node_1)
                        .or_insert_with(Vec::new)
                        .push(*node_2);
                }
            }
        }
        Self::new(new_adj_list, (0..self.num_nodes + 1).collect(), true)
    }

    fn get_all_bypass_graphs(&self) -> HashSet<(Self, Option<usize>)> {
        let mut graphs = HashSet::new();
        graphs.insert((self.clone(), None));
        while let Some((graph, _)) = utility::math::set_pop(&mut graphs) {
            let adj_list_rev = reverse_adjacency_list(&graph.adjacency_list);
            let mut node_v = None;
            for (node, edges) in &adj_list_rev {
                if edges.len() > 1 {
                    node_v = Some(node);
                    break;
                }
            }
            match node_v {
                Some(node_v) => {
                    for incoming_u in adj_list_rev.get(node_v).unwrap_or(&Vec::new()) {
                        for outgoing_w in graph.adjacency_list.get(node_v).unwrap_or(&Vec::new()) {
                            let new_graph =
                                graph.get_bypass_graph(*incoming_u, *node_v, *outgoing_w);
                            if new_graph.visited.iter().all(|x| *x) {
                                graphs.insert((new_graph, Some(*node_v)));
                            }
                        }
                    }
                }
                None => {
                    graphs.insert((graph, None));
                    break;
                }
            }
        }
        graphs
    }

    pub fn get_all_eulerian_paths(&self) -> Vec<Vec<usize>> {
        self.get_all_bypass_graphs()
            .into_iter()
            .filter_map(|(graph, node_v)| {
                if let Some(path) = graph.get_eulerian_path() {
                    match node_v {
                        Some(node_v) => Some(
                            path.into_iter()
                                .map(|n| if n == self.num_nodes { node_v } else { n })
                                .collect(),
                        ),
                        None => Some(path),
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_all_eulerian_cycles(&self) -> Vec<Vec<usize>> {
        self.get_all_bypass_graphs()
            .into_iter()
            .filter_map(|(graph, node_v)| {
                if let Some(path) = graph.get_eulerian_cycle(None) {
                    match node_v {
                        Some(node_v) => Some(
                            path.into_iter()
                                .map(|n| if n == self.num_nodes { node_v } else { n })
                                .collect(),
                        ),
                        None => Some(path),
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}
