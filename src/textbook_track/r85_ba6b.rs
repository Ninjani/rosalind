use crate::utils;
use crate::utils::Parseable;

/// Compute the Number of Breakpoints in a Permutation
///
/// Given: A signed permutation P.
///
/// Return: The number of breakpoints in P.
pub fn rosalind_ba6b() {
    let line = utils::input_from_file(
        "data/textbook_track/rosalind_ba6b.txt",
    );
    let mut permutation = vec![0];
    permutation.extend(isize::parse_line(&line[1..(line.len()-1)])
        .unwrap());
    permutation.push(permutation.len() as isize);
    println!("{}", (1..permutation.len()).filter(|i| permutation[*i] - permutation[*i-1] != 1).count())
}