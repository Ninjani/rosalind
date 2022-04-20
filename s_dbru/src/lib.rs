use std::collections::HashSet;

use anyhow::Error;

use s_revc::reverse_complement;
use std::path::Path;

/// Nodes of Bk
/// correspond to all k-mers that are present as a substring of a (k+1)-mer from S∪Src.
/// Edges of Bk
/// are encoded by the (k+1)-mers of S∪Src in the following way: for each (k+1)-mer r in S∪Src,
/// form a directed edge (r[1:k], r[2:k+1]).
//
/// Given: A collection of up to 1000 (possibly repeating) DNA strings of equal length
/// (not exceeding 50 bp) corresponding to a set S of (k+1)-mers.
///
/// Return: The adjacency list corresponding to the de Bruijn graph corresponding to S∪Src.
pub fn rosalind_dbru(filename: &Path) -> Result<HashSet<(String, String)>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let set_s: HashSet<String> = input.split('\n').map(|s| s.to_owned()).collect();
    let set_s_rc: HashSet<_> = set_s.iter().map(|s| reverse_complement(s)).collect();
    let mut adjacency_list = HashSet::new();
    for kmer in set_s.union(&set_s_rc) {
        let (kmer_l, kmer_r) = (
            kmer.chars().take(kmer.len() - 1).collect::<String>(),
            kmer.chars().skip(1).collect::<String>(),
        );
        println!("({}, {})", kmer_l, kmer_r);
        adjacency_list.insert((kmer_l, kmer_r));
    }
    Ok(adjacency_list)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn dbru() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_dbru")?;
        assert_eq!(
            rosalind_dbru(&input_file)?,
            utility::io::input_from_file(&output_file)?
                .split('\n')
                .map(|line| line[1..line.len() - 1]
                    .split(", ")
                    .map(|s| s.to_owned())
                    .collect_tuple()
                    .unwrap())
                .collect()
        );
        Ok(())
    }
}
