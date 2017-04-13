use std::io::Write;
use byteorder::{BigEndian, WriteBytesExt};
use {FarbfeldResult, FarbfeldError};

/// A farbfeld encoder
#[derive(Debug)]
pub struct FarbfeldEncoder<W: Write>(pub W);

impl<W: Write> FarbfeldEncoder<W> {
    /// Encodes a image with `width`, `height` and `data` into a farbfeld.
    /// # Failures
    /// Returns a `FarbfeldError::NotEnoughData` if the provided `data` slice is too short.
    pub fn encode(self, width: u32, height: u32, data: &[u8]) -> FarbfeldResult<()> {
        let mut w = self.0;
        let len = (width * height) as usize * 4;
        if data.len() < len { return Err(FarbfeldError::NotEnoughData) }
        w.write_all(b"farbfeld")?;
        w.write_u32::<BigEndian>(width)?;
        w.write_u32::<BigEndian>(height)?;
        w.write_all(data)?;
        Ok(())
    }
}
