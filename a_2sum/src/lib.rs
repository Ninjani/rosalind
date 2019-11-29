use failure::Error;

use utility;
use utility::io::Parseable;

/// 2SUM
///
/// Given: A positive integer k≤20, a positive integer n≤10^4, and k arrays of size n
/// containing integers from −10^5 to 10^5.
///
/// Return: For each array A[1..n], output two different indices 1≤p<q≤n such that A[p]=−A[q]
/// if exist, and "-1" otherwise.
pub fn rosalind_2sum(filename: &str) -> Result<Vec<Option<(usize, usize)>>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mut lines = input.split('\n');
    let length_input = usize::parse_line(lines.next().unwrap())?;
    let length = length_input[1];
    let mut output = Vec::with_capacity(length_input[0]);
    for line in lines {
        let mut array_indices = isize::parse_line(line)?
            .into_iter()
            .enumerate()
            .collect::<Vec<_>>();
        array_indices.sort_by(|a, b| a.1.cmp(&b.1));
        let array = array_indices.iter().map(|a| a.1).collect::<Vec<_>>();
        let indices = array_indices.iter().map(|a| a.0).collect::<Vec<_>>();
        match two_sum(length, &array, 0) {
            Some((index_1, index_2)) => {
                let mut real_indices = vec![indices[index_1] + 1, indices[index_2] + 1];
                real_indices.sort();
                println!("{}", utility::io::format_array(&real_indices));
                output.push(Some((real_indices[0], real_indices[1])));
            }
            None => {
                println!("-1");
                output.push(None);
            }
        }
    }
    Ok(output)
}

fn two_sum(length: usize, array: &[isize], target: isize) -> Option<(usize, usize)> {
    let mut index_1 = 0;
    let mut index_2 = length - 1;
    while index_1 < length {
        let sum = array[index_1] + array[index_2];
        if sum == target {
            return Some((index_1, index_2));
        }
        if sum > target {
            if index_2 >= 1 {
                index_2 -= 1;
            } else {
                break;
            }
        } else {
            index_1 += 1;
        }
        if index_1 > index_2 {
            break;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _2sum() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_2sum")?;
        let result = rosalind_2sum(&input_file)?;
        let num_nones = utility::io::input_from_file(&output_file)?
            .split('\n')
            .filter(|line| line.trim() == "-1")
            .count();
        assert_eq!(result.iter().filter(|x| x.is_none()).count(), num_nones);
        for (indices, line) in result.into_iter().zip(
            utility::io::input_from_file(&input_file)?
                .split('\n')
                .skip(1),
        ) {
            let array = isize::parse_line(line)?;
            if let Some((p, q)) = indices {
                assert!(p < q);
                assert_eq!(array[p - 1], -array[q - 1]);
            }
        }
        Ok(())
    }
}
