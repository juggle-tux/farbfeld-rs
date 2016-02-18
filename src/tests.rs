use std::io::Cursor;

use decoder::FarbfeldDecoder;
use encoder::FarbfeldEncoder;


#[test]
fn decode() {
    let buf = Cursor::new(IMAGE_DATA);
    let mut img = FarbfeldDecoder::new(buf).unwrap();
    let (w, h) = img.dimensions();
    let data = img.read_image().unwrap();
    assert_eq!(w, 3);
    assert_eq!(h, 3);
    assert_eq!(data, &IMAGE_DATA[::HEADER_LEN as usize..])
}

#[test]
fn encode() {
    let mut buf: Vec<u8>= Vec::new();
    if let Err(e) = FarbfeldEncoder(&mut buf).encode(3, 3, &IMAGE_DATA[::HEADER_LEN as usize..]) {
        panic!(e)
    }
    assert_eq!(&buf[..], IMAGE_DATA)
}

pub const IMAGE_DATA: &'static [u8] =
    b"farbfeld\
      \x00\x00\x00\x03\
      \x00\x00\x00\x03\
      \xff\x00\x00\xff\x00\xff\x00\xff\x00\x00\xff\xff\
      \x00\x00\xff\xff\x80\x80\x80\x80\x00\xff\x00\xff\
      \x00\xff\x00\xff\x00\x00\xff\xff\xff\x00\x00\xff";
