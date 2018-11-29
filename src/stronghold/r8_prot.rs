use crate::utils;

/// Get protein from RNA string (decodes till Stop codon reached)
pub fn translate(rna: &str) -> Option<String> {
    let codons = utils::get_codon_to_aa();
    let mut protein = String::with_capacity(rna.len() / 3);
    for chunk in utils::sub_strings(rna, 3) {
        match codons.get(&chunk) {
            Some(amino_acid) => {
                if amino_acid == utils::STOP_CODON_AA {
                    return Some(protein);
                } else {
                    protein.push_str(amino_acid);
                }
            }
            None => return None,
        }
    }
    None
}

/// Translating RNA into Protein
///
/// Given: An RNA string s corresponding to a strand of mRNA (of length at most 10 kbp).
///
/// Return: The protein string encoded by s.
pub fn rosalind_prot() {
    let contents = utils::input_from_file("data/stronghold/rosalind_prot.txt");
    println!("{}", translate(&contents).unwrap());
}
