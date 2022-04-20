use anyhow::Error;

use std::path::Path;

/// Enumerating Gene Orders
///
/// Given: A positive integer n≤7.
///
/// Return: The total number of permutations of length n, followed by a list of all such permutations
/// (in any order).
pub fn rosalind_perm(filename: &Path) -> Result<(usize, Vec<Vec<usize>>), Error> {
    let input = utility::io::input_from_file(filename)?;
    let n = input.parse::<usize>()?;
    let total: usize = (1..=n).product();
    let mut output = Vec::with_capacity(total);
    let mut array = (1..=n).collect::<Vec<_>>();
    println!("{}", total);
    for permutation in get_permutations(&mut array) {
        println!("{}", utility::io::format_array(&permutation));
        output.push(permutation);
    }
    Ok((total, output))
}

/// Heap's algorithm
pub fn get_permutations<T: PartialOrd + Clone>(array: &mut Vec<T>) -> Vec<Vec<T>> {
    let length = array.len();
    let mut permutations = Vec::new();
    let mut c = vec![0; length];
    permutations.push(array.clone());
    let mut i = 0;
    while i < length {
        if c[i] < i {
            if i % 2 == 0 {
                array.swap(0, i);
            } else {
                array.swap(c[i], i);
            }
            permutations.push(array.clone());
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }
    permutations
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use utility::io::Parseable;

    use super::*;

    #[test]
    fn perm() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_perm")?;
        let output = utility::io::input_from_file(&output_file)?;
        let mut output_lines = output.split('\n');
        let output_total = output_lines.next().unwrap().parse::<usize>()?;
        let output_permutations: HashSet<_> = output_lines
            .map(|line| usize::parse_line(line))
            .collect::<Result<_, _>>()?;
        let (total, permutations) = rosalind_perm(&input_file)?;
        assert_eq!(total, output_total);
        assert_eq!(
            permutations.into_iter().collect::<HashSet<_>>(),
            output_permutations
        );
        Ok(())
    }
}
