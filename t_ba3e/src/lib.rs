use anyhow::Error;

use std::path::Path;
use t_ba3d::de_bruijn_graph;

pub fn rosalind_ba3e(filename: &Path) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let patterns: Vec<_> = contents.split('\n').map(|s| s.to_owned()).collect();
    for (key, value) in de_bruijn_graph(&patterns) {
        println!("{} -> {}", key, value.join(","));
    }
    Ok(())
}
