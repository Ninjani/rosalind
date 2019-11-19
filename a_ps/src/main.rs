use std::collections::BinaryHeap;

use failure::Error;

use crate::utility;
use crate::utility::io::Parseable;

/// Partial Sort
///
/// Given: A positive integer n≤105 and an array A[1..n] of integers from −105 to 105, a positive integer k≤1000.
///
/// Return: The k smallest elements of a sorted array A.
pub fn rosalind_ps(filename: &str) -> Result<Vec<isize>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let lines: Vec<_> = input.split('\n').collect();
    let array = isize::parse_line(lines[1])?;
    let k = lines[2].parse::<usize>()?;
    let output = partial_sort(&array, k - 1);
    println!("{}", utility::io::format_array(&output));
    Ok(output)
}

fn partial_sort(array: &[isize], k: usize) -> Vec<isize> {
    let mut heap = BinaryHeap::new();
    for (i, element) in array.iter().enumerate() {
        heap.push(*element);
        if i > k {
            heap.pop();
        }
    }
    heap.into_sorted_vec()
}

#[cfg(test)]
mod tests {
    use crate::utility::io::Parseable;

    use super::*;

    #[test]
    fn ps() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_ps")?;
        let output = isize::parse_line(&utility::io::input_from_file(&output_file)?)?;
        assert_eq!(rosalind_ps(&input_file)?, output);
        Ok(())
    }
}
