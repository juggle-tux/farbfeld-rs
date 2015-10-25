use std::io::Write;
use byteorder::{BigEndian, WriteBytesExt};
use {ImgfileResult, ImgfileError};

pub struct ImagefileEncoder<W: Write>(pub W);

impl<W: Write> ImagefileEncoder<W> {
    pub fn encode(self, width: u32, height: u32, data: &[u8]) -> ImgfileResult<()> {
        let mut w = self.0;
        let len = (width * height) as usize * 4;
        if data.len() < len { return Err(ImgfileError::NotEnoughData) }
        try!(w.write_all("imagefile".as_bytes()));
        try!(w.write_u32::<BigEndian>(width));
        try!(w.write_u32::<BigEndian>(height));
        try!(w.write_all(data));
        Ok(())
    }
}
