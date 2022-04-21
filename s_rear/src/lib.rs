use anyhow::Error;

use std::path::Path;

/// Reversal Distance
///
/// Given: A collection of at most 5 pairs of permutations, all of which have length 10.
///
/// Return: The reversal distance between each permutation pair.
pub fn rosalind_rear(filename: &Path) -> Result<Vec<usize>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let mut output = Vec::new();
    for lines in input.split("\n\n") {
        let lines = lines.split("\n").collect::<Vec<_>>();
        let sequence: Vec<usize> = lines[0]
            .split(" ")
            .map(|n| n.parse())
            .collect::<Result<Vec<_>, _>>()?;
        let target_sequence: Vec<usize> = lines[1]
            .split(" ")
            .map(|n| n.parse())
            .collect::<Result<Vec<_>, _>>()?;
        output.push(find_reversal_distance(&sequence, &target_sequence)?);
    }
    println!("{}", utility::io::format_array(&output));
    Ok(output)
}

fn apply_reversal(line: &[usize], start: usize, end: usize) -> Vec<usize> {
    let mut reversed = line[..start].to_vec();
    reversed.extend(line[start..end].iter().cloned().rev());
    reversed.extend(line[end..].iter().cloned());
    reversed
}

fn find_breakpoints(sequence: &[usize], target_sequence: &[usize]) -> Result<Vec<usize>, Error> {
    let mut breakpoints = Vec::new();
    for index in 0..sequence.len() - 1 {
        let (mut p1, mut p2) = (None, None);
        for i in 0..target_sequence.len() {
            if target_sequence[i] == sequence[index] {
                p1 = Some(i);
            }
            if target_sequence[i] == sequence[index + 1] {
                p2 = Some(i);
            }
        }
        if let (Some(p1), Some(p2)) = (p1, p2) {
            if p1.abs_diff(p2) != 1 {
                breakpoints.push(index + 1);
            }
        } else {
            return Err(utility::errors::RosalindOutputError::NoneError.into());
        }
    }
    Ok(breakpoints)
}

fn find_min_breakpoint_reversals(
    sequences: Vec<Vec<usize>>,
    target_sequence: &[usize],
) -> Result<Vec<Vec<usize>>, Error> {
    let mut reversals = Vec::new();
    for sequence in sequences {
        let breakpoints = find_breakpoints(&sequence, target_sequence)?;
        if !breakpoints.is_empty() {
            for i in 0..breakpoints.len() - 1 {
                for j in i + 1..breakpoints.len() {
                    let reversal = apply_reversal(&sequence, breakpoints[i], breakpoints[j]);
                    reversals.push(reversal);
                }
            }
        }
    }
    let mut min_breakpoints = target_sequence.len();
    let mut min_breakpoint_reversals = Vec::new();
    for reversal in reversals {
        let num_breakpoints = find_breakpoints(&reversal, target_sequence)?.len();
        if num_breakpoints < min_breakpoints {
            min_breakpoints = num_breakpoints;
            min_breakpoint_reversals = vec![reversal];
        } else if num_breakpoints == min_breakpoints {
            min_breakpoint_reversals.push(reversal);
        }
    }
    Ok(min_breakpoint_reversals)
}

fn find_reversal_distance(sequence: &[usize], target_sequence: &[usize]) -> Result<usize, Error> {
    let mut sequence = sequence.to_vec();
    sequence.insert(0, 0);
    sequence.push(11);
    let mut target_sequence = target_sequence.to_vec();
    target_sequence.insert(0, 0);
    target_sequence.push(11);
    let mut reversal_distance = 0;
    let mut current_sequences = vec![sequence];
    while !current_sequences.contains(&target_sequence) {
        current_sequences = find_min_breakpoint_reversals(current_sequences, &target_sequence)?;
        reversal_distance += 1;
    }
    Ok(reversal_distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rear() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_rear")?;
        let output: Vec<usize> = utility::io::input_from_file(&output_file)?
            .split(" ")
            .map(|n| n.parse())
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(rosalind_rear(&input_file)?, output,);
        Ok(())
    }
}
