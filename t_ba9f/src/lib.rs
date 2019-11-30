use std::collections::HashMap;

use failure::Error;
use itertools::Itertools;

use t_ba9c::SuffixTree;
use t_ba9d::LongestRepeat;
use t_ba9e::{TreeColor, NodeColor};
use utility;

/// Find the Shortest Non-Shared Substring of Two Strings
///
/// Given: Strings Text1 and Text2.
///
/// Return: The shortest substring of Text1 that does not appear in Text2.
/// (Multiple solutions may exist, in which case you may return any one.)
pub fn rosalind_ba9f(filename: &str) -> Result<String, Error> {
    let contents = utility::io::input_from_file(filename)?;
    let (text_1, text_2) = contents.trim().split('\n').collect_tuple().unwrap();
    let mut text: String = text_1.to_owned();
    text.push('#');
    text.push_str(text_2);
    text.push('$');
    let suffix_tree = SuffixTree::construct(&text);
    let shortest_nonshared_substring =
        suffix_tree.get_shortest_nonshared_substring(&text, text_1.len() + 1);
    println!("{}", shortest_nonshared_substring);
    Ok(shortest_nonshared_substring)
}

pub trait ShortestNonsharedSubstring {
    fn get_shortest_nonshared_substring(&self, text: &str, split_index: usize) -> String;
}

impl ShortestNonsharedSubstring for SuffixTree {
    fn get_shortest_nonshared_substring(&self, text: &str, split_index: usize) -> String {
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
        let shallowest_node = node_depths
            .into_iter()
            .filter(|(n, d)| node_colors[n] == NodeColor::Red && *d > 0 && self.tree[*n].is_none())
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap()
            .0;
        node_ranges[&shallowest_node]
            .iter()
            .map(|e| &text[self.tree[*e].0..self.tree[*e].0 + self.tree[*e].1])
            .collect::<Vec<_>>()
            .join("")
            .to_owned()
    }
}
