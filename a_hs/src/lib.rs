use anyhow::Error;

use std::path::Path;

/// Heap Sort
///
/// Given: A positive integer n≤105 and an array A[1..n] of integers from −10^5 to 10^5.
///
/// Return: A sorted array A.
pub fn rosalind_hs(filename: &Path) -> Result<Vec<isize>, Error> {
    let (length, mut array) = utility::io::read_isize_array(filename)?;
    heap_sort(&mut array, length);
    println!("{}", utility::io::format_array(&array));
    Ok(array)
}

fn heap_sort<T: Copy + PartialOrd + PartialEq>(array: &mut [T], length: usize) {
    heapify(array, length);
    for last in (1..length).rev() {
        array.swap(last, 0);
        sift_down(array, 0, last);
    }
}

fn heapify<T: Copy + PartialEq + PartialOrd>(array: &mut [T], length: usize) {
    let mid = (((length - 1) as f64) / 2.).floor() as usize;
    for i in (0..mid).rev() {
        sift_down(array, i, length);
    }
}

fn sift_down<T: PartialOrd + PartialEq + Copy>(array: &mut [T], i: usize, length: usize) {
    let mut root = i;
    let mut left;
    let mut right;
    let mut largest;
    loop {
        largest = root;
        left = 2 * root + 1;
        right = left + 1;
        if left < length && array[left] > array[largest] {
            largest = left;
        }
        if right < length && array[right] > array[largest] {
            largest = right;
        }
        if root == largest {
            return;
        } else {
            array.swap(root, largest);
            root = largest;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hs() -> Result<(), Error> {
        let input_file = utility::testing::get_input_file("rosalind_hs")?;
        let result = rosalind_hs(&input_file)?;
        assert!((1..result.len()).all(|i| result[i - 1] <= result[i]));
        Ok(())
    }
}
