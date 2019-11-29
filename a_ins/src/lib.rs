use failure::Error;

use utility;
use utility::io::Parseable;

/// Insertion Sort
///
/// Given: A positive integer n≤10^3 and an array A[1..n] of integers.
///
/// Return: The number of swaps performed by insertion sort algorithm on A[1..n].
pub fn rosalind_ins(filename: &str) -> Result<usize, Error> {
    let input = utility::io::input_from_file(filename)?;
    let lines: Vec<_> = input.split('\n').collect();
    let length = lines[0].parse::<usize>()?;
    let mut array = isize::parse_line(lines[1])?;
    let count = insertion_sort(length, &mut array);
    println!("{}", count);
    Ok(count)
}

fn insertion_sort<T: PartialOrd>(length: usize, array: &mut [T]) -> usize {
    let mut count = 0;
    let mut k;
    for i in 1..length {
        k = i;
        while k > 0 && array[k] < array[k - 1] {
            array.swap(k - 1, k);
            count += 1;
            k -= 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ins() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_ins")?;
        let output = utility::io::input_from_file(&output_file)?.parse::<usize>()?;
        assert_eq!(rosalind_ins(&input_file)?, output);
        Ok(())
    }
}
