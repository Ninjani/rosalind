use crate::textbook_track::r47_ba3d::de_bruijn_graph;
use crate::textbook_track::r50_ba3g::get_eulerian_path;
use crate::utils;
use failure::Error;

pub fn rosalind_ba3h() -> Result<(), Error> {
    let contents = utils::input_from_file("data/textbook_track/rosalind_ba3h.txt");
    let patterns: Vec<_> = contents.split('\n').skip(1).map(|s| s.to_owned()).collect();
    let adjacency_list = de_bruijn_graph(&patterns);
    println!(
        "{}",
        get_eulerian_path(adjacency_list)
            .unwrap()
            .into_iter()
            .enumerate()
            .map(|(i, read)| if i == 0 {
                read
            } else {
                read.chars().last().unwrap().to_string()
            })
            .collect::<Vec<_>>()
            .join("")
    );
    Ok(())
}
