use failure::Error;

use crate::utility;
use crate::utility::io::Parseable;

/// nth fibonacci number given that a number is active only for m months
fn mortal_fibonacci(n: u64, m: u64) -> u64 {
    let mut fib = vec![1u64, 1u64];
    let mut gen_num = 2;
    let mut new_generation;
    while gen_num < n {
        new_generation = if gen_num < m {
            fib[fib.len() - 2] + fib[fib.len() - 1]
        } else if gen_num < m + 2 {
            fib[fib.len() - 2] + fib[fib.len() - 1] - 1
        } else {
            fib[fib.len() - 2] + fib[fib.len() - 1] - fib[fib.len() - (m + 1) as usize]
        };
        fib.push(new_generation);
        gen_num += 1;
    }
    fib[fib.len() - 1]
}

/// Mortal Fibonacci Rabbits
///
/// Given: Positive integers n≤100 and m≤20.
///
/// Return: The total number of pairs of rabbits that will remain after the nth month if all rabbits live for m months.
pub fn rosalind_fibd(filename: &str) -> Result<u64, Error> {
    let input = utility::io::input_from_file(filename)?;
    let contents = u64::parse_line(&input)?;
    let (n, m) = (contents[0], contents[1]);
    let output = mortal_fibonacci(n, m);
    println!("{}", output);
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibd() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_fibd")?;
        assert_eq!(
            rosalind_fibd(&input_file)?,
            utility::io::input_from_file(&output_file)?.parse::<u64>()?
        );
        Ok(())
    }
}
