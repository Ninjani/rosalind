use failure::Error;

use utility;

/// Building a Heap
///
/// Given: A positive integer n≤10^5 and array A[1..n] of integers from −10^5 to 10^5.
///
/// Return: A permuted array A satisfying the binary max heap property: for any 2≤i≤n, A[⌊i/2⌋]≥A[i].
pub fn rosalind_hea(filename: &str) -> Result<Vec<isize>, Error> {
    let (length, mut array) = utility::io::read_isize_array(filename)?;
    build_max_heap(&mut array, length);
    println!("{}", utility::io::format_array(&array));
    Ok(array)
}

fn build_max_heap<T: PartialOrd + PartialEq + Copy>(array: &mut [T], length: usize) {
    let mid = ((length as f64) / 2.).floor() as usize;
    for i in (0..mid).rev() {
        max_heapify(array, i)
    }
}

fn max_heapify<T: PartialOrd + PartialEq + Copy>(array: &mut [T], i: usize) {
    let left = 2 * i + 1;
    let right = left + 1;
    let mut largest = i;
    let length = array.len();
    if left < length && array[left] > array[largest] {
        largest = left;
    }
    if right < length && array[right] > array[largest] {
        largest = right;
    }
    if largest != i {
        array.swap(i, largest);
        max_heapify(array, largest);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hea() -> Result<(), Error> {
        let input_file = utility::testing::get_input_file("rosalind_hea")?;
        let result = rosalind_hea(&input_file)?;
        assert!((2..=result.len()).all(|i| result[(i / 2) - 1] >= result[i - 1]));
        Ok(())
    }
}
