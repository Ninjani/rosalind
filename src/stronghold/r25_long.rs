use crate::utils;

fn median(numbers: &mut [usize]) -> usize {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

/// Genome Assembly as Shortest Superstring
///
/// Given: At most 50 DNA strings of approximately equal length, not exceeding 1 kbp, in FASTA format (which represent reads deriving from the same strand of a single linear chromosome).
///
/// The dataset is guaranteed to satisfy the following condition: there exists a unique way to reconstruct the entire chromosome from these reads by gluing together pairs of reads that overlap by more than half their length.
///
/// Return: A shortest superstring containing all the given strings (thus corresponding to a reconstructed chromosome).
pub fn rosalind_long() {
    let mut sequences = utils::read_fasta_file("data/stronghold/rosalind_long.txt")
        .values()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    let mut chromosome = sequences[0].clone();
    while !sequences.is_empty() {
        sequences.retain(|sequence| {
            if chromosome.contains(sequence) {
                return false;
            }
            let mut found = false;
            for length in ((sequence.len() / 2 - 1)..sequence.len()).rev() {
                if chromosome.ends_with(&sequence[0..length]) {
                    chromosome += &sequence[length..];
                    found = true;
                    break;
                } else if sequence.ends_with(&chromosome[0..length]) {
                    chromosome = sequence.clone() + &chromosome[length..];
                    found = true;
                    break;
                }
            }
            !found
        });
    }
    println!("{}", chromosome);
}
