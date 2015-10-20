use std::io::{Read, Seek, SeekFrom};
use byteorder::{BigEndian, ReadBytesExt};
use image::{ColorType, DecodingResult, ImageError, ImageResult, ImageDecoder};


macro_rules! header_len(() => (9+4+4));

pub struct ImagefileDecoder<R> {
    r: R,

    width: usize,
    height: usize,
    
    scaned_lines: u64,
    is_init: bool,
}

impl<R: Read + Seek> ImagefileDecoder<R> {
    /// Create a new decoder that decodes from the stream `r`
    pub fn new(r: R) -> ImagefileDecoder<R> {
        ImagefileDecoder {
            r: r,
            width: 0,
            height: 0,
            is_init:false,
            scaned_lines: 0,
        }
    }

    fn init(&mut self) -> ImageResult<()> {
        if !self.is_init {
            try!(self.r.seek(SeekFrom::Start(0)));
            let magic = &mut [0; 9];
            try!(self.r.read_exact(magic));
            if magic != "imagefile".as_bytes() {
                return Err(ImageError::FormatError("unexpected magic number".to_string()))
            }
            
            self.width = try!(self.r.read_u32::<BigEndian>()) as usize;
            self.height = try!(self.r.read_u32::<BigEndian>()) as usize;
            self.is_init = true;
        }
        Ok(())
    }

}

impl<R: Read + Seek> ImageDecoder for ImagefileDecoder<R> {
    fn dimensions(&mut self) -> ImageResult<(u32, u32)> {
        try!(self.init());
        Ok((self.width as u32, self.height as u32))
    }
    
    fn colortype(&mut self) -> ImageResult<ColorType> {
        Ok(ColorType::RGBA(8))
    }
    
    fn row_len(&mut self) -> ImageResult<usize> {
        try!(self.init());
        Ok(4 * self.width as usize)
    }
    
    fn read_scanline(&mut self, buf: &mut [u8]) -> ImageResult<u32> {
        try!(self.init());
        if self.scaned_lines >= self.height as u64 { return Ok(0) }
        let len = self.width * 4;
        try!(self.r.seek(SeekFrom::Start(header_len!() + self.scaned_lines)));
        try!(self.r.read_exact(&mut buf[..len]));
        self.scaned_lines += 1;
        Ok(len as u32)
    }
    
    fn read_image(&mut self) -> ImageResult<DecodingResult> {
        try!(self.init());
        try!(self.r.seek(SeekFrom::Start(header_len!())));
        let num_raw_bytes = self.width * self.height * 4;
        let mut buf = Vec::with_capacity(num_raw_bytes);
        try!(self.r.read_exact(&mut buf));
        self.scaned_lines = self.height as u64;
        Ok(DecodingResult::U8(buf))
    }
}
