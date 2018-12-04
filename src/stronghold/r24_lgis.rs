use crate::utils;
use crate::utils::Parseable;
use std::iter::repeat;
use failure::Error;

/// Find the longest subsequence in a sequence of given length according to a given ordering function
fn longest_subsequence(
    length: usize,
    sequence: &[usize],
    ordering: fn(usize, usize) -> bool,
) -> Vec<usize> {
    let mut sub_length = 0usize;
    let mut ls_predecessors = Vec::new();
    let mut ls_ends = repeat(0).take(length + 1).collect::<Vec<_>>();
    for i in 0..length {
        let mut low = 1.;
        let mut high = sub_length as f64;
        while low <= high {
            let mid = ((low + high) / 2.).ceil() as usize;
            if ordering(sequence[i], sequence[ls_ends[mid]]) {
                low = (mid + 1) as f64;
            } else {
                high = (mid - 1) as f64;
            }
        }
        let new_length = low as usize;
        ls_predecessors.push(ls_ends[new_length - 1]);
        ls_ends[new_length] = i;
        if new_length > sub_length {
            sub_length = new_length;
        }
    }
    let mut subsequence = repeat(0).take(sub_length as usize).collect::<Vec<_>>();
    let mut k = ls_ends[sub_length];
    for i in (0..sub_length).rev() {
        subsequence[i] = sequence[k];
        k = ls_predecessors[k];
    }
    subsequence
}

/// Longest Increasing Subsequence
///
/// Given: A positive integer n≤10000 followed by a permutation π of length n.
///
/// Return: A longest increasing subsequence of π, followed by a longest decreasing subsequence of π.
pub fn rosalind_lgis() -> Result<(), Error> {
    let contents = utils::input_from_file("data/stronghold/rosalind_lgis.txt");
    let parts: Vec<_> = contents.split('\n').collect();
    let length = parts[0].parse::<usize>()?;
    let sequence = usize::parse_line(parts[1])?;
    utils::print_array(&longest_subsequence(length, &sequence, |x, y| x > y));
    utils::print_array(&longest_subsequence(length, &sequence, |x, y| x < y));
    Ok(())
}
