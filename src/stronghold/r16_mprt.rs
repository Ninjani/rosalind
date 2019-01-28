use crate::utils;
use failure::Error;
use regex::Regex;
use reqwest;

const UNIPROT: &str = "http://www.uniprot.org/uniprot/";

fn get_fasta_from_uniprot(uniprot_id: &str) -> Result<String, Error> {
    let url = format!("{}{}.fasta", UNIPROT, uniprot_id);
    Ok(reqwest::get(&url)?.text()?)
}

/// Strip key from fasta sequence
fn parse_sequence(sequence: &str) -> String {
    sequence.split('\n').skip(1).collect::<Vec<&str>>().join("")
}

/// Overlapping regex matcher. Returns all (1-indexed) positions where regex is found.
fn find_all(motif: &Regex, sequence: &str) -> Vec<usize> {
    let mut indices = Vec::new();
    let mut subsequence;
    let mut start_index = 0;
    loop {
        subsequence = &sequence[start_index..];
        match motif.find(subsequence) {
            Some(mat) => {
                start_index += mat.start() + 1;
                indices.push(start_index);
            }
            None => return indices,
        }
    }
}

/// Finding a Protein Motif
///
/// Given: At most 15 UniProt Protein Database access IDs.
///
/// Return: For each protein possessing the N-glycosylation motif, output its given access ID followed by a list of locations in the protein string where the motif can be found.
pub fn rosalind_mprt() -> Result<(), Error> {
    let motif = Regex::new("N[^P][ST][^P]")?;
    let contents = utils::input_from_file("data/stronghold/rosalind_mprt.txt");
    let uniprot_ids = contents.split('\n').collect::<Vec<&str>>();
    let sequences: Vec<_> = uniprot_ids
        .iter()
        .map(|key| (*key, parse_sequence(&get_fasta_from_uniprot(key).unwrap())))
        .collect();
    for (uniprot_id, sequence) in sequences {
        let indices = find_all(&motif, &sequence);
        if !indices.is_empty() {
            println!("{}", uniprot_id);
            utils::print_array(&indices);
        }
    }
    Ok(())
}
