use std::collections::btree_map::BTreeMap;
use std::collections::HashMap;

use anyhow::Error;
use petgraph::stable_graph::{EdgeIndex, EdgeReference, NodeIndex, StableGraph};
use petgraph::visit::{EdgeRef, IntoEdgeReferences};
use petgraph::{Directed, Outgoing};

use std::path::Path;
use t_ba3m::MaximalNonbranching;

/// Construct the Suffix Tree of a String
///
/// Given: A string Text.
///
/// Return: The strings labeling the edges of SuffixTree(Text).
/// (You may return these strings in any order.)
pub fn rosalind_ba9c(filename: &Path) -> Result<Vec<String>, Error> {
    let text = utility::io::input_from_file(filename)?;
    let suffix_tree = SuffixTree::construct(&text);
    let strings = suffix_tree
        .tree
        .edge_references()
        .map(|e| text[e.weight().0..e.weight().0 + e.weight().1].to_owned())
        .collect();
    for string in &strings {
        println!("{}", string);
    }
    Ok(strings)
}

pub struct SuffixTrie {
    trie: StableGraph<Option<usize>, (char, usize), Directed, u32>,
    root: NodeIndex<u32>,
}

pub struct SuffixTree {
    pub tree: StableGraph<Option<usize>, (usize, usize), Directed, u32>,
    pub root: NodeIndex<u32>,
}

impl SuffixTree {
    pub fn construct(text: &str) -> Self {
        let trie = SuffixTrie::construct(text);
        let mut tree = trie.trie.map(|_, n| *n, |_, (_, pos)| (*pos, 1));
        let root = trie.root;
        for path in trie.get_maximal_nonbranching_paths() {
            for edge in &path {
                let (source, target) = trie.trie.edge_endpoints(*edge).unwrap();
                tree.remove_edge(tree.find_edge(source, target).unwrap());
            }
            let (source, _) = trie.trie.edge_endpoints(path[0]).unwrap();
            let (_, target) = trie.trie.edge_endpoints(path[path.len() - 1]).unwrap();
            let (_, position) = trie.trie.edge_weight(path[0]).unwrap();
            tree.add_edge(source, target, (*position, path.len()));
        }
        tree.retain_nodes(|t, n| t.edges(n).next().is_some() || t[n].is_some());
        SuffixTree { tree, root }
    }
}

impl SuffixTrie {
    fn construct(text: &str) -> Self {
        let mut trie = StableGraph::new();
        let root = trie.add_node(None);
        let mut current_node;
        let text: Vec<_> = text.chars().collect();
        for i in 0..text.len() {
            current_node = root;
            for (j, current_symbol) in text.iter().enumerate().skip(i) {
                if let Some(edge) = trie
                    .edges_directed(current_node, Outgoing)
                    .find(|e: &EdgeReference<(char, usize), u32>| e.weight().0 == *current_symbol)
                {
                    current_node = edge.target();
                } else {
                    let new_node = trie.add_node(None);
                    trie.add_edge(current_node, new_node, (*current_symbol, j));
                    current_node = new_node;
                }
            }
            if trie.edges_directed(current_node, Outgoing).next().is_none() {
                *trie.node_weight_mut(current_node).unwrap() = Some(i);
            }
        }
        SuffixTrie { root, trie }
    }

    fn get_maximal_nonbranching_paths(&self) -> Vec<Vec<EdgeIndex<u32>>> {
        let nodes: Vec<_> = self.trie.node_indices().collect();
        let node_to_index: HashMap<_, _> = self
            .trie
            .node_indices()
            .enumerate()
            .map(|(i, n)| (n, i))
            .collect();
        let mut adjacency_list = BTreeMap::new();
        for e in self.trie.edge_references() {
            adjacency_list
                .entry(node_to_index[&e.source()])
                .or_insert(Vec::new())
                .push(node_to_index[&e.target()]);
        }
        let graph = utility::graph::IntegerGraph::new(
            adjacency_list,
            (0..node_to_index.len()).collect(),
            true,
        );
        let paths = graph.get_maximal_nonbranching_paths();
        paths
            .into_iter()
            .map(|path| {
                path.iter()
                    .take(path.len() - 1)
                    .zip(path.iter().skip(1))
                    .map(|(n1, n2)| self.trie.find_edge(nodes[*n1], nodes[*n2]).unwrap())
                    .collect()
            })
            .collect()
    }
}
