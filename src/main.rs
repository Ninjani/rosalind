#![allow(dead_code)]
#![allow(clippy::implicit_hasher)]
#![feature(slice_concat_ext)]
#[macro_use]
extern crate ndarray;
#[macro_use]
extern crate failure;
pub mod algorithmic_heights;
pub mod stronghold;
pub mod textbook_track;
pub mod utils;

fn main() {
    textbook_track::r85_ba6b::rosalind_ba6b()
}
