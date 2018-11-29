use crate::stronghold::r54_trie::Trie;
use crate::utils;
use std::hash::Hash;

pub fn rosalind_ba9b() {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba9b.txt");
    let mut trie = Trie::<usize, char>::new();
    let mut lines = contents.split('\n');
    let text: Vec<_> = lines.next().unwrap().chars().collect();
    for (i, line) in lines.enumerate() {
        trie.insert(&line.chars().collect::<Vec<_>>(), i)
    }
    utils::print_array(&trie.matching(&text));
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
