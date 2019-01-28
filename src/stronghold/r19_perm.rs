use crate::utils;
use failure::Error;

/// Heap's algorithm
pub fn get_permutations<T: PartialOrd + Clone>(array: &mut Vec<T>) -> Vec<Vec<T>> {
    let length = array.len();
    let mut permutations = Vec::new();
    let mut c = Vec::with_capacity(length);
    for _ in 0..length {
        c.push(0);
    }
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

/// Enumerating Gene Orders
///
/// Given: A positive integer n≤7.
///
/// Return: The total number of permutations of length n, followed by a list of all such permutations (in any order).
pub fn rosalind_perm() -> Result<(), Error> {
    let n = utils::input_from_file("data/stronghold/rosalind_perm.txt").parse::<usize>()?;
    let total: usize = (1..=n).product();
    println!("{}", total);
    let mut array = (1..=n).collect::<Vec<_>>();
    for permutation in get_permutations(&mut array) {
        utils::print_array(&permutation);
    }
    Ok(())
}
