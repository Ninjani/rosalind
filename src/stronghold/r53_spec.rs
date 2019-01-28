use crate::utils;
use failure::{err_msg, Error};

const MASS_FILE: &str = "data/monoisotopic_mass.txt";

/// Inferring Protein from Spectrum
///
/// Given: A list L of n (n≤100) positive real numbers.
///
/// Return: A protein string of length n−1 whose prefix spectrum is equal to L (if multiple solutions exist, you may output any one of them). Consult the monoisotopic mass table.
pub fn rosalind_spec() -> Result<(), Error> {
    let contents = utils::input_from_file("data/stronghold/rosalind_spec.txt");
    let spectrum = contents
        .split('\n')
        .map(|line| line.parse::<f64>())
        .collect::<Result<Vec<_>, _>>()?;
    let mass_aa = get_mass_aa()?;
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
    Ok(())
}

pub fn get_mass_aa() -> Result<Vec<(f64, char)>, Error> {
    let mass_contents = utils::input_from_file(MASS_FILE);
    let mut mass_aa = Vec::new();
    for line in mass_contents.split('\n') {
        let mut aa_mass = line.split_whitespace();
        if let (Some(aa), Some(mass)) = (aa_mass.next(), aa_mass.next()) {
            let mass = mass.parse::<f64>()?;
            let aa = aa.chars().next().ok_or_else(|| err_msg("NoneError"))?;
            mass_aa.push((mass, aa));
        }
    }
    mass_aa.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    Ok(mass_aa)
}
