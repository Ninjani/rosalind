use std::collections::HashMap;

use failure::Error;

use utility;
use utility::io::Parseable;


/// Construct the Graph of a Spectrum
/// Given: A space-delimited list of integers Spectrum.
/// Return: Graph(Spectrum).
pub fn rosalind_ba11a(filename: &str) -> Result<Vec<(usize, usize, char)>, Error> {
    let mut spectrum = vec![0];
    spectrum.append(&mut usize::parse_line(&utility::io::input_from_file(
        filename,
    )?)?);
    let mass_table = get_mass_to_aa()?;
    let graph = get_graph_spectrum(&spectrum, &mass_table);
    for (first_mass, second_mass, aa) in &graph {
        println!("{}->{}:{}", first_mass, second_mass, aa);
    }
    Ok(graph)
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

pub fn get_mass_to_aa() -> Result<HashMap<usize, char>, Error> {
    let mut mass_table = HashMap::new();
    let mass_contents = utility::io::input_from_file(utility::io::MASS_FILE)?;
    for line in mass_contents.split('\n') {
        let mut aa_mass = line.split_whitespace();
        if let (Some(aa), Some(mass)) = (aa_mass.next(), aa_mass.next()) {
            mass_table.insert(mass.parse::<f64>()? as usize, aa.chars().next().unwrap());
        }
    }
    Ok(mass_table)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use itertools::Itertools;

    use super::*;

    #[test]
    fn ba11a() -> Result<(), Error> {
        let (input_file, output_file) = utility::testing::get_input_output_file("rosalind_ba11a")?;
        let mut output = HashSet::new();
        for line in utility::io::input_from_file(&output_file)?.split('\n') {
            let (first_mass, second_mass_aa) = line.trim().split("->").collect_tuple().unwrap();
            let (second_mass, aa) = second_mass_aa.split(':').collect_tuple().unwrap();
            output.insert((
                first_mass.parse::<usize>()?,
                second_mass.parse::<usize>()?,
                aa.chars().next().unwrap(),
            ));
        }
        let result = rosalind_ba11a(&input_file)?
            .into_iter()
            .collect::<HashSet<_>>();
        assert!(result.is_subset(&output) && result.is_superset(&output));
        Ok(())
    }
}
