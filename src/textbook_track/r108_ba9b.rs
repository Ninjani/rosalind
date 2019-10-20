use std::hash::Hash;

use failure::Error;

use crate::stronghold::r54_trie::Trie;
use crate::utility;

/// Implement TrieMatching
///
/// Given: A string Text and a collection of strings Patterns.
///
/// Return: All starting positions in Text where a string from Patterns appears as a substring.
pub fn rosalind_ba9b(filename: &str) -> Result<Vec<usize>, Error> {
    let contents = utility::io::input_from_file(filename)?;
    let mut trie = Trie::<usize, char>::new();
    let mut lines = contents.split('\n');
    let text: Vec<_> = lines.next().unwrap().chars().collect();
    for (i, line) in lines.enumerate() {
        trie.insert(&line.chars().collect::<Vec<_>>(), i)
    }
    let positions = trie.matching(&text);
    println!("{}", utility::io::format_array(&positions));
    Ok(positions)
}

impl<T: Eq + Clone, U: Eq + Hash + Clone> Trie<T, U> {
    fn prefix_matching(&self, text: &[U]) -> Option<Vec<U>> {
        let mut text = text.iter();
        let mut symbol = text.next().unwrap();
        let mut v = &self.root;
        let mut pattern = Vec::new();
        loop {
            if v.children.is_empty() {
                return Some(pattern);
            } else if v.children.contains_key(&symbol) {
                pattern.push(symbol.clone());
                v = &v.children[&symbol];
                match text.next() {
                    Some(c) => symbol = c,
                    None => {
                        if v.children.is_empty() {
                            return Some(pattern);
                        } else {
                            return None;
                        }
                    }
                }
            } else {
                return None;
            }
        }
    }

    fn matching(&self, text: &[U]) -> Vec<usize> {
        let mut indices = Vec::new();
        for i in 0..text.len() {
            if self.prefix_matching(&text[i..]).is_some() {
                indices.push(i);
            }
        }
        indices
    }
}
