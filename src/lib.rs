#![feature(read_exact)]
extern crate byteorder;

mod decoder;
mod encoder;
#[cfg(test)]
mod tests;

use std::error::Error;
use std::fmt;
use std::io;

pub use decoder::ImagefileDecoder;
pub use encoder::ImagefileEncoder;

const HEADER_LEN: u64 = 9+4+4;

pub type ImgfileResult<T> = Result<T, ImgfileError>;

#[derive(Debug)]
pub enum ImgfileError {
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

impl fmt::Display for ImgfileError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &ImgfileError::FormatError(ref e) => write!(fmt, "Format error: {}", e),
            &ImgfileError::NotEnoughData => write!(fmt, "Not enough data was provided to the \
                                                         Decoder to decode the image"),
            &ImgfileError::IoError(ref e) => e.fmt(fmt),
            &ImgfileError::ImageEnd => write!(fmt, "The end of the image has been reached")
        }
    }
}

impl Error for ImgfileError {
    fn description (&self) -> &str {
        match *self {
            ImgfileError::FormatError(..) => &"Format error",
            ImgfileError::NotEnoughData(..) => &"Not enough data",
            ImgfileError::IoError(..) => &"IO error",
            ImgfileError::ImageEnd => &"Image end"
        }
    }

    fn cause (&self) -> Option<&Error> {
        match *self {
            ImgfileError::IoError(ref e) => Some(e),
            _ => None
        }
    }
}

impl From<io::Error> for ImgfileError {
    fn from(err: io::Error) -> ImgfileError {
        ImgfileError::IoError(err)
    }
}

impl From<byteorder::Error> for ImgfileError {
    fn from(err: byteorder::Error) -> ImgfileError {
        match err {
            byteorder::Error::UnexpectedEOF => ImgfileError::ImageEnd,
            byteorder::Error::Io(err) => ImgfileError::IoError(err),
        }
    }
}
