//! Farbfeld is a simple image encoding format from suckless.
//! # Related Links
//! * http://git.suckless.org/farbfeld/tree/FORMAT.
#![deny(unsafe_code)]
#![deny(trivial_casts, trivial_numeric_casts)]
#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations)]
#![deny(unused_extern_crates, unused_import_braces, unused_qualifications)]

extern crate byteorder;

use std::error;
use std::fmt;
use std::io;

mod decoder;
mod encoder;
#[cfg(test)]
mod tests;

pub use decoder::Decoder;
pub use encoder::Encoder;

const HEADER_LEN: u64 = 8+4+4;

/// Result of an image decoding/encoding process
pub type Result<T> = ::std::result::Result<T, Error>;

/// An enumeration of decoding/encoding Errors
#[derive(Debug)]
pub enum Error {
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


impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::FormatError(ref e) => write!(fmt, "Format error: {}", e),
            &Error::NotEnoughData => write!(fmt, "Not enough data was provided to the \
                                                         Decoder to decode the image"),
            &Error::IoError(ref e) => e.fmt(fmt),
            &Error::ImageEnd => write!(fmt, "The end of the image has been reached")
        }
    }
}

impl error::Error for Error {
    fn description (&self) -> &str {
        match *self {
            Error::FormatError(..) => &"Format error",
            Error::NotEnoughData => &"Not enough data",
            Error::IoError(..) => &"IO error",
            Error::ImageEnd => &"Image end"
        }
    }

    fn cause (&self) -> Option<&error::Error> {
        match *self {
            Error::IoError(ref e) => Some(e),
            _ => None
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<byteorder::Error> for Error {
    fn from(err: byteorder::Error) -> Error {
        match err {
            byteorder::Error::UnexpectedEOF => Error::ImageEnd,
            byteorder::Error::Io(err) => Error::IoError(err),
        }
    }
}
