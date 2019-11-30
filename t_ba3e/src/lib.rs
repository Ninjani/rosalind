use failure::Error;

use t_ba3d::de_bruijn_graph;
use utility;

pub fn rosalind_ba3e(filename: &str) -> Result<(), Error> {
    let contents = utility::io::input_from_file(filename)?;
    let patterns: Vec<_> = contents.split('\n').map(|s| s.to_owned()).collect();
    for (key, value) in de_bruijn_graph(&patterns) {
        println!("{} -> {}", key, value.join(","));
    }
    Ok(())
}
