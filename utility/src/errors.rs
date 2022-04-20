use std::io;
use std::num::{ParseFloatError, ParseIntError};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RosalindParseError {
    #[error("Couldn't download sample data for question: {0}")]
    SampleDataError(String),
    #[error("Couldn't read file: {file:?}")]
    FileReadError {
        #[source]
        source: io::Error,
        file: PathBuf,
    },
    #[error("Badly formatted fasta file: {0:?}")]
    BadFastaError(PathBuf),
    #[error("Badly formatted codon file (in data/codons.txt)")]
    BadCodonFileError,
    #[error("Input format error: {0}")]
    InputFormatError(String),
    #[error("Number parsing error")]
    ParseNumberError,
    #[error("Failed to parse NodeColor: {0}")]
    ParseNodeColor(String),
}

#[derive(Error, Debug)]
pub enum RosalindOutputError {
    #[error("No output")]
    NoneError,
    #[error("Number parsing error")]
    ParseNumberError,
}

impl From<ParseIntError> for RosalindParseError {
    fn from(_: ParseIntError) -> Self {
        RosalindParseError::ParseNumberError
    }
}

impl From<ParseFloatError> for RosalindParseError {
    fn from(_: ParseFloatError) -> Self {
        RosalindParseError::ParseNumberError
    }
}
