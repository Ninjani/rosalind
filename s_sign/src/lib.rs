use std::collections::HashSet;

use anyhow::Error;
use itertools::Itertools;

use s_perm::get_permutations;
use std::path::Path;

/// Enumerating Oriented Gene Orderings
///
/// Given: A positive integer n≤6.
///
/// Return: The total number of signed permutations of length n,
/// followed by a list of all such permutations (you may list the signed permutations in any order).
pub fn rosalind_sign(filename: &Path) -> Result<HashSet<Vec<i64>>, Error> {
    let length = utility::io::input_from_file(filename)?.parse::<usize>()?;
    let mut array = (1i64..=length as i64).collect::<Vec<_>>();
    let permutations = get_permutations(&mut array);
    let signs = vec![-1i64, 1];
    let number = (1usize..=length).product::<usize>() * 2usize.pow(length as u32);
    println!("{}", number);
    let mut signed_permutations = HashSet::with_capacity(number);
    for permutation in permutations {
        for sign in (0..length).map(|_| signs.iter()).multi_cartesian_product() {
            let permutation: Vec<_> = permutation.iter().zip(sign).map(|(x, y)| x * y).collect();
            println!("{}", utility::io::format_array(&permutation));
            signed_permutations.insert(permutation);
        }
    }
    Ok(signed_permutations)
}

#[cfg(test)]
mod tests {
    use utility::io::Parseable;

    use super::*;

    #[test]
    fn sign() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_sign")?;
        let output = utility::io::input_from_file(&output_file)?;
        let mut output_lines = output.split('\n');
        let result = rosalind_sign(&input_file)?;
        assert_eq!(result.len(), output_lines.next().unwrap().parse::<usize>()?);
        assert_eq!(
            result,
            output_lines
                .map(|line| i64::parse_line(line))
                .collect::<Result<HashSet<_>, _>>()?
        );
        Ok(())
    }
}
