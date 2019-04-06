#![allow(dead_code)]
#![allow(clippy::implicit_hasher)]
#![feature(slice_concat_ext)]
#![feature(vec_remove_item)]
#[macro_use]
extern crate ndarray;
#[macro_use]
extern crate failure;
pub mod algorithmic_heights;
pub mod stronghold;
pub mod textbook_track;
pub mod utils;

fn main() -> Result<(), failure::Error> {
    textbook_track::r51_ba3h::rosalind_ba3h()?;
    Ok(())
}
