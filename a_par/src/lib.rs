use anyhow::Error;

use std::path::Path;

/// 2-Way Partition
///
/// Given: A positive integer n≤105 and an array A[1..n] of integers from −10^5 to 10^5.
///
/// Return: A permuted array B[1..n] such that it is a permutation of A and there is an index 1≤q≤n
/// such that B[i]≤A[1] for all 1≤i≤q−1, B[q]=A[1], and B[i]>A[1] for all q+1≤i≤n.
pub fn rosalind_par(filename: &Path) -> Result<Vec<isize>, Error> {
    let (length, mut array) = utility::io::read_isize_array(filename)?;
    let pivot = array[0];
    partition(&mut array, length, pivot);
    println!("{}", utility::io::format_array(&array));
    Ok(array)
}

fn partition<T: PartialOrd + PartialEq + Copy>(array: &mut [T], length: usize, pivot: T) -> usize {
    let (mut i, mut j) = (0, length - 1);
    loop {
        while array[i] < pivot {
            i += 1;
        }
        while array[j] > pivot {
            j -= 1;
        }
        if i >= j {
            return j;
        }
        array.swap(i, j);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn par() -> Result<(), Error> {
        let input_file = utility::testing::get_input_file("rosalind_par")?;
        let mut result = rosalind_par(&input_file)?;
        let (length, mut input) = utility::io::read_isize_array(&input_file)?;
        assert!((1..length).any(|q| {
            (0..q - 1).all(|i| result[i] <= input[0])
                && result[q - 1] == input[0]
                && (q..length).all(|i| result[i] > input[0])
        }));
        input.sort();
        result.sort();
        assert_eq!(result, input);
        Ok(())
    }
}
