use std::collections::HashMap;

use failure::Error;
use itertools::Itertools;
use petgraph::stable_graph::NodeIndex;
use petgraph::visit::EdgeRef;

use crate::textbook_track::r110_ba9c::SuffixTree;
use utility;

/// Find the Longest Substring Shared by Two Strings
///
/// Given: Strings Text1 and Text2.
///
/// Return: The longest substring that occurs in both Text1 and Text2.
/// (Multiple solutions may exist, in which case you may return any one.)
pub fn rosalind_ba9e(filename: &str) -> Result<String, Error> {
    let contents = utility::io::input_from_file(filename)?;
    let (text_1, text_2) = contents.trim().split('\n').collect_tuple().unwrap();
    let mut text: String = text_1.to_owned();
    text.push('#');
    text.push_str(text_2);
    text.push('$');
    let suffix_tree = SuffixTree::construct(&text);
    let longest_shared_substring =
        suffix_tree.get_longest_shared_substring(&text, text_1.len() + 1);
    println!("{}", longest_shared_substring);
    Ok(longest_shared_substring)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeColor {
    Red,
    Blue,
    Purple,
    Gray,
}

impl SuffixTree {
    fn _get_children(&self, node: NodeIndex<u32>, children: &mut Vec<NodeIndex<u32>>) {
        children.push(node);
        for edge in self.tree.edges(node) {
            children.push(edge.target());
            self._get_children(edge.target(), children);
        }
    }

    fn get_children(&self, node: NodeIndex<u32>) -> Vec<NodeIndex<u32>> {
        let mut children = Vec::new();
        for edge in self.tree.edges(node) {
            self._get_children(edge.target(), &mut children);
        }
        children
    }

    pub fn tree_color(
        &self,
        initial_node_colors: HashMap<NodeIndex<u32>, NodeColor>,
    ) -> HashMap<NodeIndex<u32>, NodeColor> {
        let mut node_colors = initial_node_colors;
        let mut ripe_nodes: HashMap<NodeIndex<u32>, Vec<_>> = node_colors
            .iter()
            .filter(|(_, c)| **c == NodeColor::Gray)
            .map(|(n, _)| (*n, self.get_children(*n)))
            .filter(|(_, children)| children.iter().all(|c| node_colors[c] != NodeColor::Gray))
            .collect();
        while !ripe_nodes.is_empty() {
            for (node, children) in &ripe_nodes {
                if children.iter().all(|c| node_colors[c] == NodeColor::Red) {
                    *node_colors.entry(*node).or_insert(NodeColor::Gray) = NodeColor::Red;
                } else if children.iter().all(|c| node_colors[c] == NodeColor::Blue) {
                    *node_colors.entry(*node).or_insert(NodeColor::Gray) = NodeColor::Blue;
                } else {
                    *node_colors.entry(*node).or_insert(NodeColor::Gray) = NodeColor::Purple;
                }
            }
            ripe_nodes = node_colors
                .iter()
                .filter(|(_, c)| **c == NodeColor::Gray)
                .map(|(n, _)| (*n, self.get_children(*n)))
                .filter(|(_, children)| children.iter().all(|c| node_colors[c] != NodeColor::Gray))
                .collect();
        }
        node_colors
    }

    fn get_longest_shared_substring(&self, text: &str, split_index: usize) -> String {
        let node_colors: HashMap<_, _> = self
            .tree
            .node_indices()
            .map(|n| {
                if let Some(index) = self.tree[n] {
                    if index <= split_index {
                        (n, NodeColor::Red)
                    } else {
                        (n, NodeColor::Blue)
                    }
                } else {
                    (n, NodeColor::Gray)
                }
            })
            .collect();
        let node_colors = self.tree_color(node_colors);
        let (node_depths, node_ranges) = self.get_node_depths_edges();
        let deepest_node = node_depths
            .into_iter()
            .filter(|(n, _)| node_colors[n] == NodeColor::Purple)
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
