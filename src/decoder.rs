use std::io::{Read, Seek, SeekFrom};
use byteorder::{BigEndian, ReadBytesExt};
use image::{ColorType, DecodingResult, ImageError, ImageResult, ImageDecoder};

pub struct ImagefileDecoder<R> {
    r: R,

    width: u32,
    height: u32,
    
    decoded_rows: u32,
}

impl<R: Read + Seek> ImagefileDecoder<R> {
    /// Create a new decoder that decodes from the stream `r`
    pub fn new(r: R) -> ImageResult<ImagefileDecoder<R>> {
        let magic = &mut [0; 9];
        let mut r = r;
        try!(r.read_exact(magic));
        if magic != "imagefile".as_bytes() {
                return Err(ImageError::FormatError("unexpected magic number".to_string()))
        }

        let w = try!(r.read_u32::<BigEndian>());
        let h = try!(r.read_u32::<BigEndian>());
        Ok(ImagefileDecoder {
            r: r,
            width: w,
            height: h,
            decoded_rows: 0,
        })
    }
}

impl<R: Read + Seek> ImageDecoder for ImagefileDecoder<R> {
    fn dimensions(&mut self) -> ImageResult<(u32, u32)> {
        Ok((self.width, self.height))
    }
    
    fn colortype(&mut self) -> ImageResult<ColorType> {
        Ok(ColorType::RGBA(8))
    }
    
    fn row_len(&mut self) -> ImageResult<usize> {
        Ok(4 * self.width as usize)
    }
    
    fn read_scanline(&mut self, buf: &mut [u8]) -> ImageResult<u32> {
        if self.decoded_rows < self.height {
            try!(self.r.read_exact(&mut buf[..4 * self.width as usize]));
            self.decoded_rows += 1;
        }
        Ok(self.decoded_rows)
    }
    
    fn read_image(&mut self) -> ImageResult<DecodingResult> {
        try!(self.r.seek(SeekFrom::Start(::HEADER_LEN)));
        let (h, w) = (self.height as usize, self.width as usize);
        let num_raw_bytes = h * w * 4;
        let mut buf = Vec::with_capacity(num_raw_bytes);
        try!(self.r.read_exact(&mut buf));
        self.decoded_rows = self.height;
        Ok(DecodingResult::U8(buf))
    }
}
