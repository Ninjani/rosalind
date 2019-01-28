use crate::utils;
use failure::Error;

/// 2-Way Partition
///
/// Given: A positive integer n≤105 and an array A[1..n] of integers from −10^5 to 10^5.
///
/// Return: A permuted array B[1..n] such that it is a permutation of A and there is an index 1≤q≤n such that B[i]≤A[1] for all 1≤i≤q−1, B[q]=A[1], and B[i]>A[1] for all q+1≤i≤n.
pub fn rosalind_par() -> Result<(), Error> {
    let (length, mut array) = utils::read_isize_array("data/algorithmic_heights/rosalind_par.txt")?;
    let pivot = array[0];
    partition(&mut array, length, pivot);
    utils::print_array(&array);
    Ok(())
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
