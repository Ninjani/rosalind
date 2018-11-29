use crate::stronghold::r4_fib::fibonacci;
use crate::utils;

/// Fibonacci Numbers
///
/// Given: A positive integer n≤25
///
/// Return: The value of F_n.
pub fn rosalind_fibo() {
    let n = utils::input_from_file("data/algorithmic_heights/rosalind_fibo.txt")
        .parse::<u64>()
        .unwrap();
    println!("{}", fibonacci(n, 1));
}
