use crate::utils;
use ndarray::Array1;

/// Independent Segregation of Chromosomes
///
/// Given: A positive integer n≤50.
///
/// Return: An array A of length 2n in which A[k] represents the common logarithm of the
/// probability that two diploid siblings share at least k of their 2n chromosomes
/// (we do not consider recombination for now).
pub fn rosalind_indc() {
    let n = utils::input_from_file("data/stronghold/rosalind_indc.txt")
        .parse::<usize>()
        .unwrap()
        * 2;
    let mut log_sums = Array1::<f64>::zeros(n + 1);
    for i in 2..=n {
        log_sums[i] = (i as f64).log2() + log_sums[i - 1];
    }
    let mut probabilities = Vec::new();
    for i in 0..=n {
        probabilities.push(2f64.powf(log_sums[n] - log_sums[i] - log_sums[n - i] - n as f64));
    }
    utils::print_array(
        &(0..n)
            .map(|i| probabilities[(i + 1)..].iter().sum::<f64>().log10())
            .collect::<Vec<_>>(),
    );
}
