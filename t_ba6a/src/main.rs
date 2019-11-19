use failure::Error;

use crate::utility;
use crate::utility::io::Parseable;

/// Implement GreedySorting to Sort a Permutation by Reversals
///
/// Given: A signed permutation P.
///
/// Return: The sequence of permutations corresponding to applying GreedySorting to P,
/// ending with the identity permutation.
pub fn rosalind_ba6a() -> Result<(), Error> {
    let line = utility::io::input_from_file("data/textbook_track/rosalind_ba6a.txt")?;
    let mut permutation = isize::parse_line(&line[1..(line.len() - 1)])?;
    greedy_reversal_sorting(&mut permutation);
    Ok(())
}

fn print_permutation(permutation: &[isize]) {
    println!(
        "({})",
        permutation
            .iter()
            .map(|n| if *n > 0 {
                format!("+{}", n)
            } else {
                n.to_string()
            })
            .collect::<Vec<String>>()
            .join(" ")
    );
}

fn greedy_reversal_sorting(permutation: &mut [isize]) -> usize {
    let mut distance = 0;
    for k in 0isize..(permutation.len() as isize) {
        let k_index = k as usize;
        if permutation[k_index].abs() != (k + 1) {
            let k_position = permutation.iter().position(|x| x.abs() == k + 1).unwrap();
            permutation.swap_with_slice(
                &mut [
                    &permutation[..k_index],
                    &permutation[k_index..=k_position]
                        .iter()
                        .rev()
                        .map(|x| -x)
                        .collect::<Vec<_>>()[..],
                    &permutation[(k_position + 1)..],
                ]
                    .concat()
                    .to_vec(),
            );
            print_permutation(&permutation);
            distance += 1;
        }
        if permutation[k_index] == -(k + 1) {
            permutation[k_index] *= -1;
            print_permutation(&permutation);
            distance += 1;
        }
    }
    distance
}
