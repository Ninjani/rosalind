use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use failure::Error;

use utility;

/// Introduction to Pattern Matching
///
/// Given: A list of at most 100 DNA strings of length at most 100 bp, none of which is a prefix of another.
///
/// Return: The adjacency list corresponding to the trie T for these patterns, in the following format.
/// If T has n nodes, first label the root with 1 and then label the remaining nodes with the integers 2
/// through n in any order you like. Each edge of the adjacency list of T will be encoded by a triple
/// containing the integer representing the edge's parent node, followed by the integer representing the
/// edge's child node, and finally the symbol labeling the edge.
pub fn rosalind_trie(filename: &str) -> Result<Vec<(usize, usize, char)>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mut trie = Trie::<usize, char>::new();
    for (i, line) in input.split('\n').enumerate() {
        trie.insert(&line.chars().collect::<Vec<_>>(), i)
    }
    let mut result = Vec::new();
    for (i, j, c) in trie.traverse(&'$')[1..].iter() {
        println!("{} {} {}", i, j, c);
        result.push((*i, *j, *c));
    }
    Ok(result)
}

#[derive(Clone, Debug)]
pub struct TrieNode<T, U: Eq + Hash> {
    pub index: usize,
    pub children: HashMap<U, TrieNode<T, U>>,
    pub value: Option<T>,
}

#[derive(Clone, Debug)]
pub struct Trie<T, U: Eq + Hash + Clone> {
    pub root: TrieNode<T, U>,
    pub num_nodes: usize,
}

impl<T: Eq, U: Eq + Hash + Clone> TrieNode<T, U> {
    fn new(key: usize) -> Self {
        TrieNode {
            index: key,
            children: HashMap::new(),
            value: None,
        }
    }
}

impl<T: Eq + Clone, U: Eq + Hash + Clone> Default for Trie<T, U> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Eq + Clone, U: Eq + Hash + Clone> Trie<T, U> {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(1),
            num_nodes: 1,
        }
    }

    fn find(&self, query: &[U]) -> Option<T> {
        let node = query
            .iter()
            .fold(Some(&self.root), |current_node, c| match current_node {
                Some(n) => n.children.get(c),
                None => None,
            });
        match node {
            Some(n) => n.value.clone(),
            None => None,
        }
    }

    pub fn insert(&mut self, data: &[U], value: T) {
        let (last_node, num_nodes) = data.iter().fold(
            (&mut self.root, self.num_nodes),
            |(current_node, index), c| {
                let new_index = match current_node.children.get(c) {
                    Some(_) => index,
                    None => index + 1,
                };
                (
                    current_node
                        .children
                        .entry(c.clone())
                        .or_insert_with(|| TrieNode::new(new_index)),
                    new_index,
                )
            },
        );
        last_node.value = Some(value);
        self.num_nodes = num_nodes;
    }

    fn dfs(
        node: &TrieNode<T, U>,
        key: &U,
        parent_index: usize,
        discovered: &mut HashSet<usize>,
    ) -> Vec<(usize, usize, U)> {
        let mut edges = vec![(parent_index, node.index, key.clone())];
        discovered.insert(node.index);
        for (key, child) in &node.children {
            if !discovered.contains(&child.index) {
                edges.extend_from_slice(&Trie::dfs(child, key, node.index, discovered));
            }
        }
        edges
    }

    pub fn traverse(&self, start: &U) -> Vec<(usize, usize, U)> {
        let mut discovered = HashSet::new();
        Trie::dfs(&self.root, start, 1, &mut discovered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trie() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_trie")?;
        assert!(rosalind_trie(&input_file)?
            .into_iter()
            .zip(utility::io::input_from_file(&output_file)?.split('\n'))
            .all(|((i, j, c), y)| &format!("{} {} {}", i, j, c) == y.trim()));
        Ok(())
    }
}
