use std::collections::HashMap;
use std::path::Path;

use anyhow::Error;
use regex::Regex;

const UNIPROT_URL: &str = "http://www.uniprot.org/uniprot/";

/// Finding a Protein Motif
///
/// Given: At most 15 UniProt Protein Database access IDs.
///
/// Return: For each protein possessing the N-glycosylation motif, output its given access ID
/// followed by a list of locations in the protein string where the motif can be found.
pub async fn rosalind_mprt(filename: &Path) -> Result<HashMap<String, Vec<usize>>, Error> {
    let motif = Regex::new("N[^P][ST][^P]")?;
    let input = utility::io::input_from_file(filename)?;
    let uniprot_ids = input.split('\n').collect::<Vec<&str>>();
    let mut sequences = Vec::with_capacity(uniprot_ids.len());
    for key in uniprot_ids {
        sequences.push((key.to_owned(), get_sequence_from_uniprot(key).await?));
    }
    let output = sequences
        .into_iter()
        .filter_map(|(uniprot_id, sequence)| {
            let indices = find_all(&motif, &sequence);
            if !indices.is_empty() {
                Some((uniprot_id, indices))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();
    println!(
        "{}",
        output
            .iter()
            .map(|(k, v)| format!("{}\n{}", k, utility::io::format_array(v)))
            .collect::<Vec<_>>()
            .join("\n")
    );
    Ok(output)
}

async fn get_fasta_from_uniprot(uniprot_id: &str) -> Result<String, Error> {
    let url = format!("{}{}.fasta", UNIPROT_URL, uniprot_id);
    Ok(reqwest::get(&url).await?.text().await?)
}

async fn get_sequence_from_uniprot(uniprot_id: &str) -> Result<String, Error> {
    Ok(get_fasta_from_uniprot(uniprot_id)
        .await?
        .split('\n')
        .skip(1)
        .collect::<Vec<&str>>()
        .join(""))
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

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use utility::io::Parseable;

    use super::*;

    #[tokio::test]
    async fn mprt() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_mprt")?;
        let mut output = HashMap::new();
        for (key, positions) in utility::io::input_from_file(&output_file)?
            .split('\n')
            .tuple_windows()
        {
            output.insert(key.to_owned(), usize::parse_line(positions)?);
        }
        assert_eq!(rosalind_mprt(&input_file).await?, output);
        Ok(())
    }
}
