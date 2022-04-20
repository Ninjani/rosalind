use anyhow::Error;

use std::path::Path;
use utility::io::Parseable;

/// Binary Search
///
/// Given: Two positive integers n≤10^5 and m≤10^5, a sorted array A[1..n] of integers
/// from −10^5 to 10^5 and a list of m integers −10^5≤k_1,k_2,…,k_m≤10^5.
///
/// Return: For each k_i, output an index 1≤j≤n s.t. A[j]=k_i or "-1" if there is no such index.
pub fn rosalind_bins(filename: &Path) -> Result<Vec<isize>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let parts: Vec<_> = input.split('\n').collect();
    let array = isize::parse_line(parts[2])?;
    let keys = isize::parse_line(parts[3])?;
    let mut indices = Vec::new();
    for key in keys {
        match binary_search(0, &array, key) {
            Some(index) => indices.push((index + 1) as isize),
            None => indices.push(-1),
        }
    }
    println!("{}", utility::io::format_array(&indices));
    Ok(indices)
}

fn binary_search<T: PartialOrd + Copy>(mid: usize, array: &[T], key: T) -> Option<usize> {
    let length = array.len();
    match length {
        1 => {
            if array[0] == key {
                Some(mid)
            } else {
                None
            }
        }
        2 => {
            if array[0] == key {
                Some(mid)
            } else if array[1] == key {
                Some(mid + 1)
            } else {
                None
            }
        }
        _ => {
            let new_mid = length / 2;
            if key < array[new_mid] {
                binary_search(mid, &array[..new_mid], key)
            } else {
                binary_search(mid + new_mid, &array[new_mid..], key)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bins() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_bins")?;
        let output = isize::parse_line(&utility::io::input_from_file(&output_file)?)?;
        assert_eq!(rosalind_bins(&input_file)?, output);
        Ok(())
    }
}
