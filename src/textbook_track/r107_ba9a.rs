use crate::stronghold::r54_trie::Trie;
use crate::utils;

pub fn rosalind_ba9a() {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba9a.txt");
    let mut trie = Trie::<usize, char>::new();
    for (i, line) in contents.split('\n').enumerate() {
        trie.insert(&line.chars().collect::<Vec<_>>(), i)
    }
    for (i, j, c) in trie.traverse(&'$')[1..].iter() {
        println!("{}->{}:{}", i - 1, j - 1, c);
    }
}
