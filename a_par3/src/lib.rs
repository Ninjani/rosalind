use failure::Error;

use utility;

/// 3-Way Partition
///
/// Given: A positive integer n≤105 and an array A[1..n] of integers from −10^5 to 10^5.
///
/// Return: An array B[1..n] such that it is a permutation of A and there are indices 1≤q≤r≤n
/// such that B[i]<A[1] for all 1≤i≤q−1, B[i]=A[1] for all q≤i≤r, and B[i]>A[1] for all r+1≤i≤n.
pub fn rosalind_par3(filename: &str) -> Result<Vec<isize>, Error> {
    let (length, mut array) = utility::io::read_isize_array(filename)?;
    let pivot = array[0];
    partition(&mut array, length, pivot);
    println!("{}", utility::io::format_array(&array));
    Ok(array)
}

fn partition<T: PartialOrd + PartialEq + Copy>(array: &mut [T], length: usize, pivot: T) -> usize {
    let (mut i, mut j, mut n) = (0, 0, length - 1);
    loop {
        if j > n {
            return j;
        }
        if array[j] < pivot {
            array.swap(i, j);
            i += 1;
            j += 1;
        } else if array[j] > pivot {
            array.swap(j, n);
            n -= 1;
        } else {
            j += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Return: An array B[1..n] such that it is a permutation of A and there are indices 1≤q≤r≤n
        /// such that B[i]<A[1] for all 1≤i≤q−1, B[i]=A[1] for all q≤i≤r, and B[i]>A[1] for all r+1≤i≤n.
    #[ignore]
    #[test]
    fn par3() -> Result<(), Error> {
        let input_file = utility::testing::get_input_file("rosalind_par3")?;
        let mut result = rosalind_par3(&input_file)?;
        let (length, mut input) = utility::io::read_isize_array(&input_file)?;
        assert!((1..length).any(|q| (q..length).any(|r| {
            (0..q - 1).all(|i| result[i] < input[0])
                && ((q - 1)..r).all(|i| result[i] == input[0])
                && (r..length).all(|i| result[i] > input[0])
        })));
        input.sort();
        result.sort();
        assert_eq!(result, input);
        Ok(())
    }
}
