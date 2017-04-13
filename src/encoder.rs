use std::io::Write;
use byteorder::{BigEndian, WriteBytesExt};
use {Result, Error};

/// A farbfeld encoder
#[derive(Debug)]
pub struct Encoder<W: Write>(pub W);

impl<W: Write> Encoder<W> {
    /// Encodes a image with `width`, `height` and `data` into a farbfeld.
    /// # Failures
    /// Returns a `FarbfeldError::NotEnoughData` if the provided `data` slice is too short.
    pub fn encode(self, width: u32, height: u32, data: &[u8]) -> Result<()> {
        let mut w = self.0;
        let len = (width * height) as usize * 4;
        if data.len() < len { return Err(Error::NotEnoughData) }
        w.write_all(b"farbfeld")?;
        w.write_u32::<BigEndian>(width)?;
        w.write_u32::<BigEndian>(height)?;
        w.write_all(data)?;
        Ok(())
    }
}
