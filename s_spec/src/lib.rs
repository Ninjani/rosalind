use anyhow::Error;

use std::path::Path;

/// Inferring Protein from Spectrum
///
/// Given: A list L of n (n≤100) positive real numbers.
///
/// Return: A protein string of length n−1 whose prefix spectrum is equal to L
/// (if multiple solutions exist, you may output any one of them).
/// Consult the monoisotopic mass table.
pub fn rosalind_spec(filename: &Path) -> Result<String, Error> {
    let input = utility::io::input_from_file(filename)?;
    let spectrum = input
        .split('\n')
        .map(|line| line.parse::<f64>())
        .collect::<Result<Vec<_>, _>>()?;
    let mass_aa = utility::io::get_mass_to_aa()?;
    let mut protein = String::new();
    for i in 0..(spectrum.len() - 1) {
        let difference = spectrum[i + 1] - spectrum[i];
        for (mass, aa) in mass_aa.iter() {
            if (difference - *mass).abs() < 0.001 {
                protein.push(*aa);
                break;
            }
        }
    }
    println!("{}", protein);
    Ok(protein)
}
