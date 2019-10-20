use failure::Error;
use ndarray::Array2;

use crate::utility;

/// Find a Longest Common Subsequence of Two Strings
///
/// Given: Two strings.
///
/// Return: A longest common subsequence of these strings.
pub fn rosalind_ba5c() -> Result<(), Error> {
    let contents = utility::io::input_from_file("data/textbook_track/rosalind_ba5c.txt")?;
    let lines: Vec<_> = contents.split('\n').collect();
    println!("{}", get_longest_common_subsequence(lines[0], lines[1]));
    Ok(())
}

fn lcs_backtrack(string_1: &str, string_2: &str) -> Array2<usize> {
    let (n, m) = (string_1.len(), string_2.len());
    let mut length_lcs = Array2::<usize>::zeros((n + 1, m + 1));
    let (chars_1, chars_2): (Vec<_>, Vec<_>) =
        (string_1.chars().collect(), string_2.chars().collect());
    let mut backtrack = Array2::<usize>::zeros((n + 1, m + 1));
    for i in 1..=n {
        for j in 1..=m {
            let mut values = vec![length_lcs[(i - 1, j)], length_lcs[(i, j - 1)]];
            if chars_1[i - 1] == chars_2[j - 1] {
                values.push(length_lcs[(i - 1, j - 1)] + 1);
            }
            let (max_value, max_index) = values
                .into_iter()
                .enumerate()
                .map(|(i, a)| (a, i))
                .max()
                .unwrap();
            length_lcs[(i, j)] = max_value;
            backtrack[(i, j)] = max_index;
        }
    }
    backtrack
}

fn output_lcs(backtrack: &Array2<usize>, string_1: &[char], i: usize, j: usize) -> String {
    if i == 0 || j == 0 {
        String::new()
    } else if backtrack[(i, j)] == 0 {
        output_lcs(backtrack, string_1, i - 1, j)
    } else if backtrack[(i, j)] == 1 {
        output_lcs(backtrack, string_1, i, j - 1)
    } else {
        format!(
            "{}{}",
            output_lcs(backtrack, string_1, i - 1, j - 1),
            string_1[i - 1]
        )
    }
}

fn get_longest_common_subsequence(string_1: &str, string_2: &str) -> String {
    let backtrack = lcs_backtrack(string_1, string_2);
    output_lcs(
        &backtrack,
        &string_1.chars().collect::<Vec<_>>(),
        string_1.len(),
        string_2.len(),
    )
}
