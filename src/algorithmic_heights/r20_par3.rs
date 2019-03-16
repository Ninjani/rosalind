use crate::utils;
use failure::Error;

/// 3-Way Partition
///
/// Given: A positive integer n≤105 and an array A[1..n] of integers from −10^5 to 10^5.
///
/// Return: An array B[1..n] such that it is a permutation of A and there are indices 1≤q≤r≤n such that B[i]<A[1] for all 1≤i≤q−1, B[i]=A[1] for all q≤i≤r, and B[i]>A[1] for all r+1≤i≤n.
pub fn rosalind_par3() -> Result<(), Error> {
    let (length, mut array) =
        utils::read_isize_array("data/algorithmic_heights/rosalind_par3.txt")?;
    let pivot = array[0];
    partition(&mut array, length, pivot);
    utils::print_array(&array);
    Ok(())
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
