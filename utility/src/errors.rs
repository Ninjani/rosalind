use std::io;
use std::num::{ParseFloatError, ParseIntError};

#[derive(Fail, Debug)]
pub enum RosalindParseError {
    #[fail(display = "Couldn't read file: {}", _1)]
    FileReadError(#[fail(cause)] io::Error, String),
    #[fail(display = "Badly formatted fasta file: {}", _0)]
    BadFastaError(String),
    #[fail(display = "Badly formatted codon file (in data/codons.txt)")]
    BadCodonFileError,
    #[fail(display = "Input format error: {}", _0)]
    InputFormatError(String),
    #[fail(display = "Number parsing error")]
    ParseNumberError,
    #[fail(display = "Failed to parse NodeColor: {}", _0)]
    ParseNodeColor(String),
}

#[derive(Fail, Debug)]
pub enum RosalindOutputError {
    #[fail(display = "No output")]
    NoneError,
    #[fail(display = "Number parsing error")]
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
