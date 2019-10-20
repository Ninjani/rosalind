use std::collections::HashMap;

use failure::Error;

use crate::utility;

/// Consensus and Profile
///
/// Given: A collection of at most 10 DNA strings of equal length (at most 1 kbp) in FASTA format.
///
/// Return: A consensus string and profile matrix for the collection.
/// (If several possible consensus strings exist, then you may return any one of them.)
pub fn rosalind_cons(filename: &str) -> Result<(String, Vec<HashMap<char, usize>>), Error> {
    let contents = utility::io::read_fasta_file(filename)?;
    let sequences = contents.values().map(|s| s.as_ref()).collect::<Vec<&str>>();
    let profile = get_profile(&sequences);
    let consensus = get_consensus(&profile);
    println!("{}\n{}", consensus, format_profile(&profile));
    Ok((consensus, profile))
}

/// Get frequencies of each nucleotide at each position in a collection of sequences (profile)
fn get_profile(sequences: &[&str]) -> Vec<HashMap<char, usize>> {
    let sequence_length = sequences[0].len();
    let mut profile = Vec::with_capacity(sequences.len());
    for i in 0..sequence_length {
        let position_string = sequences
            .iter()
            .map(|sequence| sequence.chars().nth(i).unwrap())
            .collect::<String>();
        profile.push(utility::string::char_counter(&position_string));
    }
    profile
}

/// Get consensus sequence from a profile
fn get_consensus(profile: &[HashMap<char, usize>]) -> String {
    let mut consensus = String::with_capacity(profile.len());
    for counts in profile.iter() {
        let mut count_vec: Vec<_> = counts.iter().collect();
        count_vec.sort_by(|a, b| b.1.cmp(a.1));
        consensus.push(*count_vec[0].0);
    }
    consensus
}

/// Profile pretty-printer
fn format_profile(profile: &[HashMap<char, usize>]) -> String {
    let mut output = Vec::new();
    for nucleotide in "ACGT".chars() {
        output.push(format!(
            "{}: {}",
            nucleotide,
            profile
                .iter()
                .map(|counts| counts.get(&nucleotide).unwrap_or(&0).to_string())
                .collect::<Vec<String>>()
                .join(" ")
        ));
    }
    output.join("\n")
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::utility::io::Parseable;

    use super::*;

    fn read_profile(
        lines: &mut Iterator<Item=&str>,
        length: usize,
    ) -> Result<Vec<HashMap<char, usize>>, Error> {
        let mut profile: Vec<_> = (0..length).map(|_| HashMap::with_capacity(4)).collect();
        let char_order = "ACGT".chars().collect::<Vec<_>>();
        for (i, line) in lines.enumerate() {
            let (character, counts) = line.split(": ").collect_tuple().unwrap();
            assert_eq!(character.chars().next().unwrap(), char_order[i]);
            for (j, count) in usize::parse_line(counts)?.into_iter().enumerate() {
                profile[j].insert(char_order[i], count);
            }
        }
        Ok(profile)
    }

    #[test]
    fn cons() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_cons")?;
        let output = utility::io::input_from_file(&output_file)?;
        let mut output_lines = output.split('\n');
        let output_consensus = output_lines.next().unwrap().trim();
        let output_profile = read_profile(&mut output_lines, output_consensus.len())?;
        let (consensus, profile) = rosalind_cons(&input_file)?;
        assert_eq!(consensus.len(), output_consensus.len());
        for i in 0..output_profile.len() {
            for character in "ACGT".chars() {
                assert_eq!(
                    profile[i].get(&character).unwrap_or(&0),
                    output_profile[i].get(&character).unwrap_or(&0)
                );
            }
        }
        Ok(())
    }
}
