use failure::Error;

use utility;

pub fn rosalind_ba3c(filename: &str) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let kmers: Vec<_> = contents.split('\n').collect();
    let overlap_graph = get_overlap_graph(&kmers, kmers[0].len() - 1);
    for (kmer_1, kmer_2) in overlap_graph {
        println!("{} -> {}", kmer_1, kmer_2);
    }
    Ok(())
}

pub fn get_overlap_graph(sequences: &[&str], overlap_length: usize) -> Vec<(String, String)> {
    let nodes = sequences
        .iter()
        .map(|sequence| {
            let length = sequence.len();
            (
                *sequence,
                &sequence[0..overlap_length],
                &sequence[(length - overlap_length)..],
            )
        })
        .collect::<Vec<(&str, &str, &str)>>();
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
