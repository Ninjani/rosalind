use std::collections::HashMap;

use failure::Error;

use s_lexf::enumerate_lex;
use utility;

/// k-Mer Composition
///
/// Given: A DNA string s in FASTA format (having length at most 100 kbp).
///
/// Return: The 4-mer composition of s.
pub fn rosalind_kmer(filename: &str) -> Result<Vec<Vec<usize>>, Error> {
    let alphabets = vec!['A', 'C', 'G', 'T'];
    let dna = utility::io::read_fasta_file(filename)?;
    let kmer_indices: HashMap<String, usize> = enumerate_lex(&alphabets, 4)
        .into_iter()
        .enumerate()
        .map(|(i, k)| (k, i))
        .collect();
    let mut counts: Vec<Vec<_>> = Vec::with_capacity(dna.len());
    for (i, (_, sequence)) in dna.into_iter().enumerate() {
        counts.push((0..kmer_indices.len()).map(|_| 0).collect());
        for kmer in utility::string::kmerize(&sequence, 4) {
            counts[i][kmer_indices[&kmer]] += 1;
        }
        println!("{}", utility::io::format_array(&counts[i]));
    }
    Ok(counts)
}

#[cfg(test)]
mod tests {
    use utility::io::Parseable;

    use super::*;

    #[test]
    fn kmer() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_kmer")?;
        let output = utility::io::input_from_file(&output_file)?
            .split('\n')
            .map(|line| usize::parse_line(line))
            .collect::<Result<Vec<Vec<_>>, _>>()?;
        assert_eq!(rosalind_kmer(&input_file)?, output);
        Ok(())
    }
}
