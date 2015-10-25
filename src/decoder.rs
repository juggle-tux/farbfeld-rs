use std::io::{Read, Seek, SeekFrom};
use byteorder::{BigEndian, ByteOrder};
use {HEADER_LEN, ImgfileError, ImgfileResult};

pub struct ImagefileDecoder<R> {
    r: R,

    width: u32,
    height: u32,
}

impl<R: Read + Seek> ImagefileDecoder<R> {
    /// Create a new decoder that decodes from the stream `r`
    pub fn new(r: R) -> ImgfileResult<ImagefileDecoder<R>> {
        let mut r = r;
        try!(r.seek(SeekFrom::Start(0)));
        let head = &mut [0; HEADER_LEN as usize];
        try!(r.read_exact(head));
        if &head[..9] != "imagefile".as_bytes() {
                return Err(ImgfileError::FormatError("unexpected magic number".to_string()))
        }

        Ok(ImagefileDecoder {
            r: r,
            width: BigEndian::read_u32(&head[9..]),
            height: BigEndian::read_u32(&head[13..]),
        })
    }
}

impl<R: Read + Seek> ImagefileDecoder<R> {
    pub fn dimensions(&mut self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn row_len(&mut self) -> usize {
        self.width as usize * 4
    }
    
    pub fn read_row(&mut self, row: u32,  buf: &mut [u8]) -> ImgfileResult<usize> {
        if row > self.height { return Err(ImgfileError::ImageEnd) }

        let row_len = self.row_len();
        let offset = HEADER_LEN + row as u64 * row_len as u64;
        try!(self.r.seek(SeekFrom::Start(offset)));
        try!(self.r.read_exact(&mut buf[..row_len]));
        Ok(row_len)
    }

    pub fn read_image(&mut self) -> ImgfileResult<Vec<u8>> {
        try!(self.r.seek(SeekFrom::Start(HEADER_LEN)));
        let num_raw_bytes = self.height as usize * self.row_len();
        let mut buf = vec![0; num_raw_bytes];
        try!(self.r.read_exact(&mut buf));
        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Cursor, ErrorKind, Write};
    use ImagefileDecoder;
    use tests::{IMAGE_DATA};
    use ImgfileError;

    macro_rules! try_panic {
        ($e:expr) => {
            match $e {
                Err(e) => panic!(e),
                Ok(res) => res,
            }
        }
    }

    #[test]
    fn invalid_magic() {
        let mut img_data = Vec::new();
        try_panic!(img_data.write(b"test fail"));
        try_panic!(img_data.write(&IMAGE_DATA[9..]));
        let buf = Cursor::new(img_data);

        match ImagefileDecoder::new(buf) {
            Err(e) => match e {
                ImgfileError::FormatError(_) => return,
                e => panic!("{:?}", e),
            },
            Ok(_) => panic!("Got Ok expected ImgfileError::FormatError"),
        }
    }

    #[test]
    fn truncate_header() {
        let buf = Cursor::new(&IMAGE_DATA[0..8]);

        match ImagefileDecoder::new(buf) {
            Err(ImgfileError::IoError(e)) => {
                if e.kind() == ErrorKind::UnexpectedEOF {
                    return
                } else { panic!("{:?}", e) }
            },
            Err(e) => panic!("{:?}", e),
            Ok(_) => panic!("Got Ok expected ImgfileError::FormatError"),
        }

    }

    #[test]
    fn truncate_data() {
        let mut img_data = Vec::with_capacity(IMAGE_DATA.len()-1);
        try_panic!(img_data.write_all(&IMAGE_DATA[..IMAGE_DATA.len()-1]));
        let buf = Cursor::new(img_data);
        let mut img = ImagefileDecoder::new(buf).unwrap();
        match img.read_image() {
            Err(ImgfileError::IoError(e)) => {
                if e.kind() == ErrorKind::UnexpectedEOF {
                    return
                } else { panic!("{:?}", e) }
            },
            Err(e) => panic!("{:?}", e),
            Ok(_) => panic!("Got Ok expected ImgfileError::FormatError"),
        }
    }
}
