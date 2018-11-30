#![allow(dead_code)]
#![allow(clippy::implicit_hasher)]
#[macro_use]
extern crate ndarray;

pub mod algorithmic_heights;
pub mod stronghold;
pub mod textbook_track;
pub mod utils;

fn main() {
    textbook_track::r68_ba4l::rosalind_ba4l()
}
