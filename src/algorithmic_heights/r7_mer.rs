use failure::Error;

use crate::utility;
use crate::utility::io::Parseable;

/// Merge Two Sorted Arrays
///
/// Given: A positive integer n≤10^5 and a sorted array A[1..n] of integers from −10^5 to 10^5, a positive integer m≤105 and a sorted array B[1..m] of integers from −10^5 to 10^5.
///
/// Return: A sorted array C[1..n+m] containing all the elements of A and B.
pub fn rosalind_mer(filename: &str) -> Result<Vec<isize>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let lines: Vec<_> = input.split('\n').collect();
    let sublist_1 = isize::parse_line(lines[1])?;
    let sublist_2 = isize::parse_line(lines[3])?;
    let merged_list = merge(&sublist_1, &sublist_2);
    println!("{}", utility::io::format_array(&merged_list));
    Ok(merged_list)
}

pub fn merge<T: PartialOrd + PartialEq + Copy>(left_array: &[T], right_array: &[T]) -> Vec<T> {
    let mut i = 0;
    let mut j = 0;
    let mut sorted_array = Vec::new();
    while i < left_array.len() && j < right_array.len() {
        if left_array[i] <= right_array[j] {
            sorted_array.push(left_array[i]);
            i += 1;
        } else {
            sorted_array.push(right_array[j]);
            j += 1;
        }
    }
    sorted_array.extend(&left_array[i..]);
    sorted_array.extend(&right_array[j..]);
    sorted_array
}

#[cfg(test)]
mod tests {
    use crate::utility::io::Parseable;

    use super::*;

    #[test]
    fn mer() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_mer")?;
        let output = isize::parse_line(&utility::io::input_from_file(&output_file)?)?;
        assert_eq!(rosalind_mer(&input_file)?, output);
        Ok(())
    }
}
