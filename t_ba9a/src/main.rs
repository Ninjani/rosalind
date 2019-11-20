use failure::Error;

use crate::stronghold::r54_trie::Trie;
use crate::utility;

/// Construct a Trie from a Collection of Patterns
///
/// Given: A collection of strings Patterns.
///
/// Return: The adjacency list corresponding to Trie(Patterns), in the following format.
/// If Trie(Patterns) has n nodes, first label the root with 1 and then label the remaining nodes
/// with the integers 2 through n in any order you like. Each edge of the adjacency list of
/// Trie(Patterns) will be encoded by a triple: the first two members of the triple must be the
/// integers labeling the initial and terminal nodes of the edge, respectively; the third member
/// of the triple must be the symbol labeling the edge.
pub fn rosalind_ba9a(filename: &str) -> Result<Vec<(usize, usize, char)>, Error> {
    let contents = utility::io::input_from_file(filename)?;
    let mut trie = Trie::<usize, char>::new();
    for (i, line) in contents.split('\n').enumerate() {
        trie.insert(&line.chars().collect::<Vec<_>>(), i)
    }
    let adjacency_list = trie.traverse(&'$').into_iter().skip(1).collect();
    for &(i, j, c) in &adjacency_list {
        println!("{}->{}:{}", i - 1, j - 1, c);
    }
    Ok(adjacency_list)
}
