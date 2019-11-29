use failure::Error;

use utility;
use utility::io::Parseable;

/// Longest Increasing Subsequence
///
/// Given: A positive integer n≤10000 followed by a permutation π of length n.
///
/// Return: A longest increasing subsequence of π, followed by a longest decreasing subsequence of π.
pub fn rosalind_lgis(filename: &str) -> Result<(Vec<usize>, Vec<usize>), Error> {
    let input = utility::io::input_from_file(filename)?;
    let parts: Vec<_> = input.split('\n').collect();
    let length = parts[0].parse::<usize>()?;
    let sequence = usize::parse_line(parts[1])?;
    let inc_subs = longest_subsequence(length, &sequence, |x, y| x > y);
    let dec_subs = longest_subsequence(length, &sequence, |x, y| x < y);
    println!(
        "{}\n{}",
        utility::io::format_array(&inc_subs),
        utility::io::format_array(&dec_subs)
    );
    Ok((inc_subs, dec_subs))
}

/// Find the longest subsequence in a sequence of given length according to a given ordering function
fn longest_subsequence(
    length: usize,
    sequence: &[usize],
    ordering: fn(usize, usize) -> bool,
) -> Vec<usize> {
    let mut sub_length = 0usize;
    let mut ls_predecessors = Vec::new();
    let mut ls_ends = (0..=length).map(|_| 0).collect::<Vec<_>>();
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
    let mut subsequence = (0..sub_length as usize).map(|_| 0).collect::<Vec<_>>();
    let mut k = ls_ends[sub_length];
    for i in (0..sub_length).rev() {
        subsequence[i] = sequence[k];
        k = ls_predecessors[k];
    }
    subsequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lgis() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_lgis")?;
        let output_lines = utility::io::input_from_file(&output_file)?
            .split('\n')
            .filter(|line| !line.trim().is_empty())
            .map(|line| usize::parse_line(line))
            .collect::<Result<Vec<_>, _>>()?;
        let (inc_subs, dec_subs) = rosalind_lgis(&input_file)?;
        assert_eq!(inc_subs.len(), output_lines[0].len());
        assert_eq!(dec_subs.len(), output_lines[1].len());
        assert!((1..inc_subs.len()).all(|i| inc_subs[i] > inc_subs[i - 1]));
        assert!((1..dec_subs.len()).all(|i| dec_subs[i] < dec_subs[i - 1]));
        Ok(())
    }
}
