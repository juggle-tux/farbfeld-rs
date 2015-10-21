use image::{Rgba, RgbaImage};
use std::io::Cursor;

use decoder::ImagefileDecoder;
use encoder::ImagefileEncoder;


#[test]
fn decode() {
    let buf = Cursor::new(IMAGE_DATA);
    let img = ImagefileDecoder::new(buf).unwrap().into_img().unwrap();
    assert_eq!(img.into_raw(), test_image().into_raw())
}

#[test]
fn encode() {
    let mut buf: Vec<u8>= Vec::new();
    if let Err(e) = ImagefileEncoder(&mut buf).encode_img(test_image()) {
        panic!(e)
    }
    assert_eq!(&buf[..], IMAGE_DATA)
}

pub const IMAGE_DATA: &'static [u8] =
    b"imagefile\
      \x00\x00\x00\x03\
      \x00\x00\x00\x03\
      \xff\x00\x00\xff\x00\xff\x00\xff\x00\x00\xff\xff\
      \x00\x00\xff\xff\x80\x80\x80\x80\x00\xff\x00\xff\
      \x00\xff\x00\xff\x00\x00\xff\xff\xff\x00\x00\xff";

pub const RED: Rgba<u8> = Rgba{data: [255, 0, 0 ,255]};
pub const GREEN: Rgba<u8> = Rgba{data: [0, 255, 0, 255]};
pub const BLUE: Rgba<u8> = Rgba{data: [0, 0, 255, 255]};
pub const GRAY: Rgba<u8> = Rgba{data: [128, 128, 128, 128]};

pub fn test_image() -> RgbaImage {
    let img: [Rgba<u8>; 3*3] = [RED, GREEN, BLUE,
                                BLUE, GRAY, GREEN,
                                GREEN, BLUE, RED];
    let mut buf = RgbaImage::new(3,3);
    buf.pixels_mut().zip(img.iter())
        .map(|(b, i)| {*b = *i}).count();
    return buf
}
