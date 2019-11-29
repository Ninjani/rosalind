use failure::Error;
use rand::{Rng, thread_rng};

use crate::algorithmic_heights::r24_med::partition;
use utility;
use utility::io::Parseable;

/// Quick Sort
///
/// Given: A positive integer n≤105 and an array A[1..n] of integers from −105 to 105
///
/// Return: A sorted array A[1..n].
pub fn rosalind_qs(filename: &str) -> Result<Vec<isize>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let lines: Vec<_> = input.split('\n').collect();
    let mut array = isize::parse_line(lines[1])?;
    quicksort(&mut array);
    println!("{}", utility::io::format_array(&array));
    Ok(array)
}

fn quicksort(array: &mut [isize]) {
    let mut pivot_index = thread_rng().gen_range(0, array.len());
    pivot_index = partition(array, 0, array.len() - 1, pivot_index);
    if pivot_index == 0 {
        pivot_index += 1;
    }
    let (mut left_array, mut right_array) = array.split_at_mut(pivot_index);
    if left_array.len() > 1 {
        quicksort(&mut left_array);
    }
    if right_array.len() > 1 {
        quicksort(&mut right_array);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qs() -> Result<(), Error> {
        let input_file = utility::testing::get_input_file("rosalind_qs")?;
        let array = rosalind_qs(&input_file)?;
        assert!((1..array.len()).all(|i| array[i - 1] <= array[i]));
        Ok(())
    }
}
