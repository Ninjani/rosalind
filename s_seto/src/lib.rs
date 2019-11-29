use std::collections::HashSet;
use std::iter::FromIterator;

use failure::Error;

use crate::utility;

/// Introduction to Set Operations
///
/// Given: A positive integer n (n≤20,000) and two subsets A and B of {1,2,…,n}.
///
/// Return: Six sets: A∪B, A∩B, A−B, B−A, Ac, and Bc
/// (where set complements are taken with respect to {1,2,…,n}).
pub fn rosalind_seto(filename: &str) -> Result<Vec<HashSet<usize>>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let lines: Vec<_> = input.split('\n').collect();
    let max_n = lines[0].parse::<usize>()?;
    let set_a = utility::io::read_set(&lines[1])?;
    let set_b = utility::io::read_set(&lines[2])?;
    let set_u = HashSet::from_iter(1..=max_n);
    let result = vec![
        set_a.union(&set_b).cloned().collect(),
        set_a.intersection(&set_b).cloned().collect(),
        set_a.difference(&set_b).cloned().collect(),
        set_b.difference(&set_a).cloned().collect(),
        set_u.difference(&set_a).cloned().collect(),
        set_u.difference(&set_b).cloned().collect(),
    ];
    for set in &result {
        println!("{}", utility::io::format_set(&set));
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seto() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_seto")?;
        let result = rosalind_seto(&input_file)?;
        for (i, set) in utility::io::input_from_file(&output_file)?
            .split('\n')
            .enumerate()
            {
                assert_eq!(result[i], utility::io::read_set(set)?);
            }
        Ok(())
    }
}
