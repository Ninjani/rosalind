use failure::Error;

use crate::utility;

/// Creating a Character Table from Genetic Strings
///
/// Given: A collection of at most 100 characterizable DNA strings, each of length at most 300 bp.
///
/// Return: A character table for which each nontrivial character encodes the symbol choice at a
/// single position of the strings. (Note: the choice of assigning '1' and '0' to the two states
/// of each SNP in the strings is arbitrary.)
pub fn rosalind_cstr(filename: &str) -> Result<(), Error> {
    let input = utility::io::input_from_file(filename)?;
    let strings: Vec<Vec<_>> = input.split('\n').map(|s| s.chars().collect()).collect();
    let num_strings = strings.len();
    let table: Vec<_> = (0..num_strings)
        .map(|i| {
            let one = strings[0][i];
            strings
                .iter()
                .map(|s| if s[i] == one { 1 } else { 0 })
                .collect::<Vec<_>>()
        })
        .collect();
    for row in table {
        let row_sum = row.iter().sum::<usize>();
        // Filter out trivial splits
        if row_sum > 1 && row_sum < num_strings - 1 {
            println!(
                "{}",
                row.into_iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            );
        }
    }
    Ok(())
}
