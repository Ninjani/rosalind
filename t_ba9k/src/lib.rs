use std::collections::HashMap;
use std::hash::Hash;
use std::path::Path;

use anyhow::Error;

pub fn rosalind_ba9k(filename: &Path) -> Result<usize, Error> {
    let contents = utility::io::input_from_file(filename)?;
    let mut lines = contents.split('\n');
    let transform: Vec<_> = lines.next().unwrap().chars().collect();
    let index = lines.next().unwrap().parse::<usize>()?;
    let mut first_column: Vec<_> = transform.to_vec();
    first_column.sort_unstable();
    let last_to_first = get_last_to_first_mapping(&transform, &first_column);
    let first_index = last_to_first[&index];
    println!("{}", first_index);
    Ok(first_index)
}

pub fn get_last_to_first_mapping<T: Hash + PartialEq + Eq>(
    last: &[T],
    first: &[T],
) -> HashMap<usize, usize> {
    let mut mapping_last = HashMap::new();
    for (i, c) in last.iter().enumerate() {
        mapping_last.entry(c).or_insert_with(Vec::new).push(i);
    }
    let mut mapping_first = HashMap::new();
    for (i, c) in first.iter().enumerate() {
        mapping_first.entry(c).or_insert_with(Vec::new).push(i);
    }
    let mut last_to_first = HashMap::with_capacity(last.len());
    for (element, last_indices) in mapping_last {
        for (last_index, first_index) in
            last_indices.into_iter().zip(mapping_first[&element].iter())
        {
            last_to_first.insert(last_index, *first_index);
        }
    }
    last_to_first
}
