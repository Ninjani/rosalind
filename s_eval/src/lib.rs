use failure::Error;

use s_prob::nucleotide_probs_from_gc_content;
use utility;
use utility::io::Parseable;

/// Expected Number of Restriction Sites
///
/// Given: A positive integer n (n≤1,000,000), a DNA string s of even length at most 10,
/// and an array A of length at most 20, containing numbers between 0 and 1.
///
/// Return: An array B having the same length as A in which B[i] represents the expected number of
/// times that s will appear as a substring of a random DNA string t of length n,
/// where t is formed with GC-content A[i].
pub fn rosalind_eval(filename: &str) -> Result<Vec<f64>, Error> {
    let input = utility::io::input_from_file(filename)?;
    let lines: Vec<_> = input.split('\n').collect();
    let length = lines[0].parse::<usize>()?;
    let substring = lines[1];
    let gc_contents = f64::parse_line(lines[2])?;
    let mut b = Vec::with_capacity(gc_contents.len());
    for gc_content in gc_contents {
        let nucleotide_probs = nucleotide_probs_from_gc_content(gc_content);
        let probability_substring = substring
            .chars()
            .map(|c| nucleotide_probs[&c])
            .product::<f64>();
        b.push(probability_substring * ((length - substring.len() + 1) as f64));
    }
    println!("{}", utility::io::format_array(&b));
    Ok(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_eval")?;
        assert!(rosalind_eval(&input_file)?
            .into_iter()
            .zip(f64::parse_line(&utility::io::input_from_file(&output_file)?)?.into_iter())
            .all(|(x, y)| (x - y).abs() < utility::testing::ROSALIND_FLOAT_ERROR_F64));
        Ok(())
    }
}
