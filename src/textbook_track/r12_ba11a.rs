use crate::utils;
use crate::utils::Parseable;
use std::collections::HashMap;
use failure::Error;

const MASS_FILE: &str = "data/monoisotopic_mass.txt";

/// Construct the Graph of a Spectrum
/// Given: A space-delimited list of integers Spectrum.
/// Return: Graph(Spectrum).
pub fn rosalind_ba11a() -> Result<(), Error> {
    let mut spectrum = vec![0];
    spectrum.append(
        &mut usize::parse_line(&utils::input_from_file(
            "data/textbook_track/rosalind_ba11a.txt",
        ))?,
    );
    let mass_table = get_mass_to_aa();
    let graph = get_graph_spectrum(&spectrum, &mass_table);
    for (first_mass, second_mass, aa) in graph {
        println!("{}->{}:{}", first_mass, second_mass, aa);
    }
    Ok(())
}

pub fn get_graph_spectrum(
    spectrum: &[usize],
    mass_table: &HashMap<usize, char>,
) -> Vec<(usize, usize, char)> {
    let mut adjacency_list = Vec::new();
    for i in 0..(spectrum.len() - 1) {
        for j in (i + 1)..spectrum.len() {
            let (first_mass, second_mass) = (spectrum[i], spectrum[j]);
            if let Some(aa) = mass_table.get(&(second_mass - first_mass)) {
                adjacency_list.push((first_mass, second_mass, *aa));
            }
        }
    }
    adjacency_list
}

pub fn get_mass_to_aa() -> HashMap<usize, char> {
    let mut mass_table = HashMap::new();
    let mass_contents = utils::input_from_file(MASS_FILE);
    for line in mass_contents.split('\n') {
        let mut aa_mass = line.split_whitespace();
        if let (Some(aa), Some(mass)) = (aa_mass.next(), aa_mass.next()) {
            mass_table.insert(
                mass.parse::<f64>().unwrap() as usize,
                aa.chars().next().unwrap(),
            );
        }
    }
    mass_table
}
