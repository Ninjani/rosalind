#![allow(dead_code)]
#![allow(clippy::implicit_hasher)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::deref_addrof)]
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
    color_backtrace::install();
    textbook_track::r6_ba10f::rosalind_ba10f()?;
    Ok(())
}
