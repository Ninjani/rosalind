use failure::Error;
use ndarray::Array1;

use utility;

/// Independent Segregation of Chromosomes
///
/// Given: A positive integer n≤50.
///
/// Return: An array A of length 2n in which A[k] represents the common logarithm of the
/// probability that two diploid siblings share at least k of their 2n chromosomes
/// (we do not consider recombination for now).
pub fn rosalind_indc(filename: &str) -> Result<Vec<f64>, Error> {
    let n = utility::io::input_from_file(filename)?.parse::<usize>()? * 2;
    let mut log_sums = Array1::<f64>::zeros(n + 1);
    for i in 2..=n {
        log_sums[i] = (i as f64).log2() + log_sums[i - 1];
    }
    let mut probabilities = Vec::new();
    for i in 0..=n {
        probabilities.push(2f64.powf(log_sums[n] - log_sums[i] - log_sums[n - i] - n as f64));
    }
    let result: Vec<_> = (0..n)
        .map(|i| probabilities[(i + 1)..].iter().sum::<f64>().log10())
        .collect();
    println!("{}", utility::io::format_array(&result));
    Ok(result)
}

#[cfg(test)]
mod tests {
    use utility::io::Parseable;

    use super::*;

    #[test]
    fn indc() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_indc")?;
        assert!(rosalind_indc(&input_file)?
            .into_iter()
            .zip(f64::parse_line(&utility::io::input_from_file(
                &output_file
            )?)?)
            .all(|(x, y)| (x - y).abs() < utility::testing::ROSALIND_FLOAT_ERROR_F64));
        Ok(())
    }
}
