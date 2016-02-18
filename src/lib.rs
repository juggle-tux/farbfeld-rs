//! Farbfeld is a simple image encoding format from suckless.
//! # Related Links
//! * http://git.suckless.org/farbfeld/tree/FORMAT.
#![deny(unsafe_code)]
#![deny(trivial_casts, trivial_numeric_casts)]
#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations)]
#![deny(unused_extern_crates, unused_import_braces, unused_qualifications)]

extern crate byteorder;

pub use decoder::FarbfeldDecoder;
pub use encoder::FarbfeldEncoder;

/// Result of an image decoding/encoding process
pub type FarbfeldResult<T> = Result<T, FarbfeldError>;

/// An enumeration of decoding/encoding Errors
#[derive(Debug)]
pub enum FarbfeldError {
     /// The Image is not formatted properly
    FormatError(String),

    /// Not enough data was provided to the Decoder
    /// to decode the image
    NotEnoughData,

    /// An I/O Error occurred while decoding the image
    IoError(io::Error),

    /// The end of the image has been reached
    ImageEnd
}

const HEADER_LEN: u64 = 8+4+4;

mod decoder;
mod encoder;
#[cfg(test)]
mod tests;

use std::error::Error;
use std::fmt;
use std::io;

impl fmt::Display for FarbfeldError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &FarbfeldError::FormatError(ref e) => write!(fmt, "Format error: {}", e),
            &FarbfeldError::NotEnoughData => write!(fmt, "Not enough data was provided to the \
                                                         Decoder to decode the image"),
            &FarbfeldError::IoError(ref e) => e.fmt(fmt),
            &FarbfeldError::ImageEnd => write!(fmt, "The end of the image has been reached")
        }
    }
}

impl Error for FarbfeldError {
    fn description (&self) -> &str {
        match *self {
            FarbfeldError::FormatError(..) => &"Format error",
            FarbfeldError::NotEnoughData => &"Not enough data",
            FarbfeldError::IoError(..) => &"IO error",
            FarbfeldError::ImageEnd => &"Image end"
        }
    }

    fn cause (&self) -> Option<&Error> {
        match *self {
            FarbfeldError::IoError(ref e) => Some(e),
            _ => None
        }
    }
}

impl From<io::Error> for FarbfeldError {
    fn from(err: io::Error) -> FarbfeldError {
        FarbfeldError::IoError(err)
    }
}

impl From<byteorder::Error> for FarbfeldError {
    fn from(err: byteorder::Error) -> FarbfeldError {
        match err {
            byteorder::Error::UnexpectedEOF => FarbfeldError::ImageEnd,
            byteorder::Error::Io(err) => FarbfeldError::IoError(err),
        }
    }
}
