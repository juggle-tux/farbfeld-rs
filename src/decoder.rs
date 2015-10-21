use std::io::{Read, Seek, SeekFrom};
use byteorder::{BigEndian, ReadBytesExt};
use image::{ColorType, DecodingResult, ImageError, ImageResult, ImageDecoder, RgbaImage};

pub struct ImagefileDecoder<R> {
    r: R,

    width: u32,
    height: u32,
    
    row_len: usize,
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
            row_len: w as usize * 4,
        })
    }

    pub fn into_img(self) -> ImageResult<RgbaImage> {
        let mut img = self;
        match  try!(img.read_image()) {
            DecodingResult::U8(data) => {
                let (w, h) = try!(img.dimensions());
                println!("w: {:?}, h: {:?}, data len: {:?}", w, h, data.len());
                return Ok(RgbaImage::from_raw(w, h, data).expect("failed to load ImageBuffer"))
            },
            _ => Err(ImageError::NotEnoughData),
        }
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
        Ok(self.row_len)
    }
    
    fn read_scanline(&mut self, buf: &mut [u8]) -> ImageResult<u32> {
        let rows = self.decoded_rows;
        if rows < self.height {
            let offset = ::HEADER_LEN + rows as u64 * self.row_len as u64;
            try!(self.r.seek(SeekFrom::Start(offset)));
            try!(self.r.read_exact(&mut buf[..self.row_len]));
            self.decoded_rows += 1;
        }
        Ok(self.decoded_rows)
    }
    
    fn read_image(&mut self) -> ImageResult<DecodingResult> {
        try!(self.r.seek(SeekFrom::Start(::HEADER_LEN)));
        let num_raw_bytes = self.height as usize * self.row_len;
        let mut buf = vec![0; num_raw_bytes];
        try!(self.r.read_exact(&mut buf));
        self.decoded_rows = self.height;
        Ok(DecodingResult::U8(buf))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use ImagefileDecoder;
    use tests::{IMAGE_DATA};
    use image::{ImageDecoder, ImageError};

    macro_rules! try_panic {
        ($e:expr) => {
            match $e {
                Err(e) => panic!(e),
                Ok(res) => res,
            }
        }
    }

    use std::io::Write;
    #[test]
    fn invalid_magic() {
        let mut img_data = Vec::new();
        try_panic!(img_data.write(b"testfail."));
        try_panic!(img_data.write(&IMAGE_DATA[9..]));
        let buf = Cursor::new(img_data);

        match ImagefileDecoder::new(buf) {
            Err(e) => match e {
                ImageError::FormatError(_) => return,
                e => panic!("{:?}", e),
            },
            Ok(_) => panic!("Got Ok expected ImageError::FormatError"),
        }
    }

    #[test]
    fn truncate_header() {
        let buf = Cursor::new(&IMAGE_DATA[0..8]);

        match ImagefileDecoder::new(buf) {
            Err(e) => match e {
                ImageError::IoError(_) => return,
                e => panic!("{:?}", e),
            },
            Ok(_) => panic!("Got Ok expected ImageError::FormatError"),
        }

    }

    #[test]
    fn truncate_data() {
        let mut img_data = Vec::new();
        try_panic!(img_data.write(&IMAGE_DATA[..IMAGE_DATA.len()-2]));
        let buf = Cursor::new(img_data);
        let mut img =ImagefileDecoder::new(buf).unwrap();
        match img.read_image() {
            Err(ImageError::IoError(_)) => return,
            Err(e) => panic!("{:?}", e),
            Ok(_) => panic!("Got Ok expected ImageError::FormatError"),
        }
    }
}
