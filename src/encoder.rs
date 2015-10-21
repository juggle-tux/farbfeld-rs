use std::io::Write;
use byteorder::{BigEndian, WriteBytesExt};
use image::{ColorType, GenericImage, ImageError, ImageResult, RgbaImage};

pub struct ImagefileEncoder<W: Write>(pub W);

impl<W: Write> ImagefileEncoder<W> {
    pub fn encode(self, data: &[u8], width: u32, height: u32, color: ColorType) -> ImageResult<()> {
        let mut w = self.0;
        try!(w.write_all("imagefile".as_bytes()));
        try!(w.write_u32::<BigEndian>(width));
        try!(w.write_u32::<BigEndian>(height));
        let ib: RgbaImage = match color {
            ColorType::RGBA(8) => RgbaImage::from_raw(width, height, data.into()).unwrap(),
            c => {
                return Err(ImageError::UnsupportedColor(c))
            }
        };
        try!(w.write_all(&ib.into_raw().into_boxed_slice()));
        Ok(())
    }

    pub fn encode_img(self, img: RgbaImage) -> ImageResult<()>{
        let (w, h) = img.dimensions();
        self.encode(
            &img.into_raw().into_boxed_slice(),
            w, h, ::COLOR_TYPE)
    }
}

