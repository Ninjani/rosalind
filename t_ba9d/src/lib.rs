use std::collections::HashMap;

use failure::Error;
use petgraph::Outgoing;
use petgraph::stable_graph::{EdgeIndex, NodeIndex};
use petgraph::stable_graph::EdgeReference;
use petgraph::visit::EdgeRef;

use crate::textbook_track::r110_ba9c::SuffixTree;
use utility;

/// Find the longest repeat in a string.
///
/// Given: A string Text.
///
/// Return: A longest substring of Text that appears in Text more than once.
/// (Multiple solutions may exist, in which case you may return any one.)
pub fn rosalind_ba9d(filename: &str) -> Result<String, Error> {
    let mut text = utility::io::input_from_file(filename)?;
    text.push('$');
    let suffix_tree = SuffixTree::construct(&text);
    let longest_repeat = suffix_tree.get_longest_repeat(&text);
    println!("{}", longest_repeat);
    Ok(longest_repeat)
}

impl SuffixTree {
    fn _get_node_depths_edges(
        &self,
        edge: EdgeReference<(usize, usize), u32>,
        depth: usize,
        node_depths: &mut HashMap<NodeIndex<u32>, usize>,
        node_paths: &mut HashMap<NodeIndex<u32>, Vec<EdgeIndex<u32>>>,
    ) {
        node_depths.insert(edge.target(), depth);
        let mut path = node_paths[&edge.source()].clone();
        path.push(edge.id());
        node_paths.insert(edge.target(), path);
        for next_edge in self.tree.edges_directed(edge.target(), Outgoing) {
            self._get_node_depths_edges(
                next_edge,
                depth + next_edge.weight().1,
                node_depths,
                node_paths,
            );
        }
    }

    pub fn get_node_depths_edges(
        &self,
    ) -> (
        HashMap<NodeIndex<u32>, usize>,
        HashMap<NodeIndex<u32>, Vec<EdgeIndex<u32>>>,
    ) {
        let mut node_depths: HashMap<_, _> = HashMap::new();
        let mut node_paths: HashMap<_, _> = HashMap::new();
        node_depths.insert(self.root, 0);
        node_paths.insert(self.root, Vec::new());
        for edge in self.tree.edges_directed(self.root, Outgoing) {
            self._get_node_depths_edges(edge, edge.weight().1, &mut node_depths, &mut node_paths);
        }
        (node_depths, node_paths)
    }

    fn get_longest_repeat(&self, text: &str) -> String {
        let (node_depths, node_ranges) = self.get_node_depths_edges();
        let deepest_node = node_depths
            .into_iter()
            .filter(|(n, _)| self.tree[*n].is_none())
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap()
            .0;
        node_ranges[&deepest_node]
            .iter()
            .map(|e| &text[self.tree[*e].0..self.tree[*e].0 + self.tree[*e].1])
            .collect::<Vec<_>>()
            .join("")
            .to_owned()
    }
}
