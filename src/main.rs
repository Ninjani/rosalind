#![allow(dead_code)]
#![allow(clippy::implicit_hasher)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::deref_addrof)]
#![feature(slice_concat_ext)]
#![feature(vec_remove_item)]
#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate failure;
#[macro_use]
extern crate ndarray;

pub mod algorithmic_heights;
pub mod stronghold;
pub mod textbook_track;
pub mod utility;

fn main() -> Result<(), failure::Error> {
    color_backtrace::install();
    //    utility::testing::get_all_sample_data("test_data/small")?;
    //    textbook_track::r119_ba9l::rosalind_ba9l("test_data/small/rosalind_ba9l.txt").unwrap();
    textbook_track::r119_ba9l::rosalind_ba9l("data/textbook_track/rosalind_ba9l.txt").unwrap();
    Ok(())
}
