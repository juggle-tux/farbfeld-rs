#![feature(read_exact)]

extern crate image;
extern crate byteorder;

pub type Pixel = image::Rgba<u8>;

pub use decoder::ImagefileDecoder;
pub use encoder::ImagefileEncoder;

const HEADER_LEN: u64 = 9+4+4;
const COLOR_TYPE: image::ColorType = image::ColorType::RGBA(8);

mod decoder;
mod encoder;
#[cfg(test)]
mod tests;
