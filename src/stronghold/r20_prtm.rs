use crate::utils;
use failure::Error;
use hashbrown::HashMap;

const MASS_FILE: &str = "data/monoisotopic_mass.txt";

/// Reads monoisotopic mass table
pub fn get_aa_to_mass() -> Result<HashMap<char, f64>, Error> {
    let mut mass_table = HashMap::new();
    let mass_contents = utils::input_from_file(MASS_FILE);
    for line in mass_contents.split('\n') {
        let mut aa_mass = line.split_whitespace();
        if let (Some(aa), Some(mass)) = (aa_mass.next(), aa_mass.next()) {
            mass_table.insert(aa.chars().next().unwrap(), mass.parse::<f64>()?);
        }
    }
    Ok(mass_table)
}

/// Calculating Protein Mass
///
/// Given: A protein string P of length at most 1000 aa.
///
/// Return: The total weight of P. Consult the monoisotopic mass table.
pub fn rosalind_prtm() -> Result<(), Error> {
    let protein = utils::input_from_file("data/stronghold/rosalind_prtm.txt");
    let mass_table = get_aa_to_mass()?;
    println!("{}", protein.chars().map(|c| &mass_table[&c]).sum::<f64>());
    Ok(())
}
