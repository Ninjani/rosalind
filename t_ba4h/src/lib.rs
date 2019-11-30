use failure::Error;
use itertools::Itertools;

use t_ba4e::spectrum_list_to_counts;
use utility;
use utility::io::Parseable;

/// Generate the Convolution of a Spectrum
///
/// Given: A collection of integers Spectrum.
///
/// Return: The list of elements in the convolution of Spectrum in decreasing order of their
/// multiplicities. If an element has multiplicity k, it should appear exactly k times.
pub fn rosalind_ba4h(filename: &str) -> Result<(), Error> {
    let spectrum = usize::parse_line(&utility::io::input_from_file(
        filename,
    )?)?;
    let convolution = get_spectral_convolution(&spectrum);
    let convolution_counts = spectrum_list_to_counts(&convolution);
    let order: Vec<_> = convolution_counts
        .into_iter()
        .sorted_by(|a, b| b.1.cmp(&a.1))
        .into_iter()
        .flat_map(|(mass, count)| (0..count).map(move |_| mass))
        .collect();
    println!("{}", utility::io::format_array(&order));
    Ok(())
}

pub fn get_spectral_convolution(spectrum: &[usize]) -> Vec<usize> {
    let mut spectrum = spectrum.to_vec();
    spectrum.sort();
    let mut convolution = Vec::new();
    for i in 0..(spectrum.len() - 1) {
        for j in i..spectrum.len() {
            if spectrum[j] > spectrum[i] {
                convolution.push(spectrum[j] - spectrum[i]);
            }
        }
    }
    convolution
}
