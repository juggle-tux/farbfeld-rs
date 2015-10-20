#![feature(read_exact)]
extern crate image;
extern crate byteorder;

pub use decoder::ImagefileDecoder;
pub use encoder::ImagefileEncoder;

mod decoder;
mod encoder;
pub const HEADER_LEN: u64 = 9+4+4;
