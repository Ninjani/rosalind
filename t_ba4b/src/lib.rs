use std::collections::HashMap;

use failure::Error;

use s_rna::transcribe;
use s_revc::reverse_complement;
use utility;

pub fn rosalind_ba4b(filename: &str) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let codons = utility::io::get_codon_to_aa()?;
    let lines: Vec<_> = contents.split('\n').collect();
    let (dna, protein) = (lines[0], lines[1]);
    let num_nucleotides = protein.len() * 3;
    for i in 0..(dna.len() - num_nucleotides) {
        let current_dna = &dna[i..(i + num_nucleotides)];
        let revc_dna = reverse_complement(current_dna);
        let (rna, revc_rna) = (transcribe(&current_dna), transcribe(&revc_dna));
        match (
            translate_no_stop(&rna, &codons),
            translate_no_stop(&revc_rna, &codons),
        ) {
            (Some(p1), Some(p2)) => {
                if p1 == protein || p2 == protein {
                    println!("{}", current_dna);
                }
            }
            (Some(p1), _) => {
                if p1 == protein {
                    println!("{}", current_dna);
                }
            }
            (_, Some(p2)) => {
                if p2 == protein {
                    println!("{}", current_dna);
                }
            }
            _ => (),
        }
    }
    Ok(())
}

pub fn translate_no_stop(rna: &str, codons: &HashMap<String, String>) -> Option<String> {
    let mut protein = String::with_capacity(rna.len() / 3);
    for chunk in utility::string::sub_strings(rna, 3) {
        match codons.get(&chunk) {
            Some(amino_acid) => {
                if amino_acid == utility::io::STOP_CODON_AA {
                    return None;
                } else {
                    protein.push_str(amino_acid);
                }
            }
            None => return None,
        }
    }
    Some(protein)
}
