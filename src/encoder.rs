extern crate byteorder;

use std::io::{self, Write};
use image::{ColorType, ImageBuffer, RgbaImage};
use self::byteorder::{BigEndian, WriteBytesExt};

pub struct ImagefileEncoder<W: Write> {
    w: W,
}

impl<W: Write> ImagefileEncoder<W> {
    pub fn encode(self, data: &[u8], width: u32, height: u32, color: ColorType) -> io::Result<()> {
        let mut w = self.w;
        try!(w.write_all("imagefile".as_bytes()));
        try!(w.write_u32::<BigEndian>(width));
        try!(w.write_u32::<BigEndian>(height));
        let ib: RgbaImage = match color {
            ColorType::RGBA(8) => ImageBuffer::from_raw(width, height, data.into()).unwrap(),
            c => {
                panic!(c)
            }
        };
        try!(w.write_all(&ib.into_raw().into_boxed_slice()));
        Ok(())
    }

}
