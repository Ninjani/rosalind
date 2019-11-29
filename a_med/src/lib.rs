use failure::Error;
use rand::{Rng, thread_rng};

use utility;
use utility::io::Parseable;

/// Median
///
/// Given: A positive integer n≤105 and an array A[1..n] of integers
/// from −10^5 to 10^5, a positive number k≤n.
///
/// Return: The k-th smallest element of A.
pub fn rosalind_med(filename: &str) -> Result<isize, Error> {
    let input = utility::io::input_from_file(filename)?;
    let lines: Vec<_> = input.split('\n').collect();
    let length = lines[0].parse::<usize>()?;
    let mut array = isize::parse_line(lines[1])?;
    let k = lines[2].parse::<usize>()?;
    let output = select(&mut array, 0, length - 1, k - 1);
    println!("{}", output);
    Ok(output)
}

pub fn partition(array: &mut [isize], left: usize, right: usize, pivot_index: usize) -> usize {
    let pivot = array[pivot_index];
    array.swap(pivot_index, right);
    let mut store_index = left;
    for i in left..right {
        if array[i] < pivot {
            array.swap(store_index, i);
            store_index += 1;
        }
    }
    array.swap(right, store_index);
    store_index
}

fn select(array: &mut [isize], left: usize, right: usize, k: usize) -> isize {
    let mut pivot_index;
    let (mut left, mut right) = (left, right);
    loop {
        if left == right {
            return array[left];
        }
        pivot_index = thread_rng().gen_range(left, right);
        pivot_index = partition(array, left, right, pivot_index);
        if pivot_index == k {
            return array[k];
        } else if pivot_index < k {
            left = pivot_index + 1;
        } else {
            right = pivot_index - 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn med() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_med")?;
        let output = utility::io::input_from_file(&output_file)?.parse::<isize>()?;
        assert_eq!(rosalind_med(&input_file)?, output);
        Ok(())
    }
}
