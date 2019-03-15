use crate::stronghold::r3_revc::reverse_complement;
use crate::stronghold::r6_hamm::hamming;
use crate::utils;
use hashbrown::HashMap;

/// Error Correction in Reads
///
/// Given: A collection of up to 1000 reads of equal length (at most 50 bp) in FASTA format. Some of these reads were generated with a single-nucleotide error. For each read s in the dataset, one of the following applies:
///     s was correctly sequenced and appears in the dataset at least twice (possibly as a reverse complement);
///     s is incorrect, it appears in the dataset exactly once, and its Hamming distance is 1 with respect to exactly one correct read in the dataset (or its reverse complement).
///
/// Return: A list of all corrections in the form "[old read]->[new read]". (Each correction must be a single symbol substitution, and you may return the corrections in any order.)
pub fn rosalind_corr() {
    let fasta = utils::read_fasta_file("data/stronghold/rosalind_corr.txt");
    let reads: Vec<String> = fasta.values().map(|x| x.to_owned()).collect();
    let mut counter = HashMap::new();
    let mut correct_reads = Vec::new();
    for read in reads.into_iter() {
        if let Some(value) = counter.get_mut(&read) {
            *value += 1;
            correct_reads.push(read);
            continue;
        }
        if let Some(value) = counter.get_mut(&reverse_complement(&read)) {
            *value += 1;
            correct_reads.push(read);
            continue;
        }
        counter.insert(read, 1usize);
    }
    for read in counter.keys().filter(|k| counter[*k] == 1) {
        for correct_read in correct_reads.iter() {
            if hamming(read, correct_read) == 1 {
                println!("{}->{}", read, correct_read);
                break;
            } else {
                let correct_read_revc = reverse_complement(correct_read);
                if hamming(read, &correct_read_revc) == 1 {
                    println!("{}->{}", read, correct_read_revc);
                    break;
                }
            }
        }
    }
}
