use std::collections::HashMap;

use failure::Error;
use itertools::Itertools;

use crate::utility;
use crate::utility::io::Parseable;

/// Comparing Spectra with the Spectral Convolution
///
/// Given: Two multisets of positive real numbers S1 and S2. The size of each multiset is at most 200.
///
/// Return: The largest multiplicity of S1⊖S2, as well as the absolute value of the number x
/// maximizing (S1⊖S2)(x) (you may return any such value if multiple solutions exist).
pub fn rosalind_conv(filename: &str) -> Result<(usize, Vec<f64>), Error> {
    let input = utility::io::input_from_file(filename)?;
    let (line_1, line_2) = input.split('\n').collect_tuple().unwrap();
    let multiset_a = f64::parse_line(line_1)?;
    let multiset_b = f64::parse_line(line_2)?;
    let multiset_a_b = get_minkowski_difference(&multiset_a, &multiset_b);
    let (multiplicity, xs) = get_max_multiplicity(&multiset_a_b)?;
    println!("{}\n{}", multiplicity, xs[0]);
    Ok((multiplicity, xs))
}

pub fn get_max_multiplicity(multiset: &[f64]) -> Result<(usize, Vec<f64>), Error> {
    let mut counts = HashMap::new();
    for s in multiset {
        *counts.entry(format!("{:.5}", s)).or_insert(0usize) += 1;
    }
    let mut counts_tuple = counts.into_iter().collect::<Vec<_>>();
    counts_tuple.sort_by(|a, b| b.1.cmp(&a.1));
    let x_max = counts_tuple[0].0.parse::<f64>()?;
    let mut x_maxes = vec![x_max];
    let mut i = 1;
    while counts_tuple[i].0.parse::<f64>()? == x_maxes[0] {
        x_maxes.push(counts_tuple[i].0.parse::<f64>()?);
        i += 1;
    }
    Ok((counts_tuple[0].1, x_maxes))
}

pub fn get_minkowski_difference(multiset_s1: &[f64], multiset_s2: &[f64]) -> Vec<f64> {
    let mut multiset_s1_s2 = Vec::new();
    for s1 in multiset_s1 {
        for s2 in multiset_s2 {
            multiset_s1_s2.push(*s1 - *s2);
        }
    }
    multiset_s1_s2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conv() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_conv")?;
        let (multiplicity, xs) = rosalind_conv(&input_file)?;
        let output = utility::io::input_from_file(&output_file)?;
        let mut output_lines = output.split('\n');
        assert_eq!(multiplicity, output_lines.next().unwrap().parse::<usize>()?);
        let output_x = output_lines.next().unwrap().parse::<f64>()?;
        assert!(xs
            .into_iter()
            .any(|x| (x - output_x).abs() < utility::testing::ROSALIND_FLOAT_ERROR_F64));
        Ok(())
    }
}
