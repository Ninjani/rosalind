use anyhow::Error;

use std::path::Path;

/// Calculating Protein Mass
///
/// Given: A protein string P of length at most 1000 aa.
///
/// Return: The total weight of P. Consult the monoisotopic mass table.
pub fn rosalind_prtm(filename: &Path) -> Result<f64, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mass_table = utility::io::get_aa_to_mass()?;
    let output = input.chars().map(|c| &mass_table[&c]).sum::<f64>();
    println!("{}", output);
    Ok(output)
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    #[test]
    fn prtm() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_prtm")?;
        let output = utility::io::input_from_file(&output_file)?.parse::<f64>()?;
        assert_approx_eq!(
            rosalind_prtm(&input_file)?,
            output,
            utility::testing::ROSALIND_FLOAT_ERROR_F64
        );
        Ok(())
    }
}
