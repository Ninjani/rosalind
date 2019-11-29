use std::collections::HashMap;

use failure::Error;

use utility;

/// Overlap Graphs
///
/// Given: A collection of DNA strings in FASTA format having total length at most 10 kbp.
///
/// Return: The adjacency list corresponding to O_3. You may return edges in any order.
pub fn rosalind_grph(filename: &str) -> Result<Vec<(String, String)>, Error> {
    let sequences = utility::io::read_fasta_file(filename)?;
    let overlap_length = 3;
    let output: Vec<(String, String)> = get_overlap_graph(&sequences, overlap_length)
        .into_iter()
        .collect();
    println!(
        "{}",
        output
            .iter()
            .map(|(key_0, key_1)| format!("{} {}", key_0, key_1))
            .collect::<Vec<_>>()
            .join("\n")
    );
    Ok(output)
}

/// Make graph connecting sequences overlapping by overlap_length
pub fn get_overlap_graph(
    sequences: &HashMap<String, String>,
    overlap_length: usize,
) -> Vec<(String, String)> {
    let nodes = sequences
        .iter()
        .map(|(key, sequence)| {
            let length = sequence.len();
            let prefix = &sequence[0..overlap_length];
            let suffix = &sequence[(length - overlap_length)..];
            (key, prefix, suffix)
        })
        .collect::<Vec<(&String, &str, &str)>>();
    let mut edges = Vec::new();
    for i in 0..nodes.len() {
        for j in 0..nodes.len() {
            if i != j {
                let (key_0, _, suffix_0) = nodes[i];
                let (key_1, prefix_1, _) = nodes[j];
                if suffix_0 == prefix_1 {
                    edges.push((key_0.to_owned(), key_1.to_owned()));
                }
            }
        }
    }
    edges
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use itertools::Itertools;

    use super::*;

    #[test]
    fn grph() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_grph")?;
        let output_edges: HashSet<_> = utility::io::input_from_file(&output_file)?
            .split('\n')
            .map(|line| {
                let (s1, s2) = line.split_whitespace().collect_tuple().unwrap();
                (s1.to_owned(), s2.to_owned())
            })
            .collect();
        assert_eq!(
            rosalind_grph(&input_file)?
                .into_iter()
                .collect::<HashSet<_>>(),
            output_edges
        );
        Ok(())
    }
}
