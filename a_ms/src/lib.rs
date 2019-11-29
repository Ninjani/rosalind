use failure::Error;

use a_mer::merge;
use utility;

/// Merge Sort
///
/// Given: A positive integer n≤105 and an array A[1..n] of integers from −10^5 to 10^5.
///
/// Return: A sorted array A[1..n].
pub fn rosalind_ms(filename: &str) -> Result<Vec<isize>, Error> {
    let (_, array) = utility::io::read_isize_array(filename)?;
    let sorted_array = merge_sort(&array);
    println!("{}", utility::io::format_array(&sorted_array));
    Ok(sorted_array)
}

fn merge_sort<T: PartialOrd + PartialEq + Copy>(array: &[T]) -> Vec<T> {
    let length = array.len();
    if length > 1 {
        let mid = length / 2;
        merge(&merge_sort(&array[..mid]), &merge_sort(&array[mid..]))
    } else {
        array.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ms() -> Result<(), Error> {
        let input_file = utility::testing::get_input_file("rosalind_ms")?;
        let result = rosalind_ms(&input_file)?;
        assert!((1..result.len()).all(|i| result[i - 1] <= result[i]));
        Ok(())
    }
}
