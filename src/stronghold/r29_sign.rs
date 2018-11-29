use crate::stronghold::r19_perm::get_permutations;
use crate::utils;
use itertools::Itertools;

/// Enumerating Oriented Gene Orderings
///
/// Given: A positive integer n≤6.
///
/// Return: The total number of signed permutations of length n, followed by a list of all such permutations (you may list the signed permutations in any order).
pub fn rosalind_sign() {
    let length = utils::input_from_file("data/stronghold/rosalind_sign.txt")
        .parse::<usize>()
        .unwrap();
    let mut array = (1i64..=length as i64).collect::<Vec<_>>();
    let permutations = get_permutations(&mut array);
    let signs = vec![-1i64, 1];
    println!(
        "{}",
        (1usize..=length).product::<usize>() * 2usize.pow(length as u32)
    );
    for permutation in permutations {
        for sign in (0..length).map(|_| signs.iter()).multi_cartesian_product() {
            utils::print_array(
                &permutation
                    .iter()
                    .zip(sign)
                    .map(|(x, y)| x * y)
                    .collect::<Vec<_>>(),
            );
        }
    }
}
