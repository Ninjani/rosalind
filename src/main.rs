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
    algorithmic_heights::r16_dag::rosalind_dag();
    Ok(())
}
