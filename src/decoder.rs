use std::io::{Read, Seek, SeekFrom};
use byteorder::{BigEndian, ByteOrder};
use {HEADER_LEN, FarbfeldError, FarbfeldResult};

/// A farbfeld decoder
#[derive(Debug)]
pub struct FarbfeldDecoder<R> {
    r: R,

    width: u32,
    height: u32,
}

impl<R: Read + Seek> FarbfeldDecoder<R> {
    /// Create a new decoder from `r` and parse the header.
    /// # Failures
    /// Returns FarbfeldError::FormatError if the magic number does not match `farbfeld`
    pub fn new(r: R) -> FarbfeldResult<FarbfeldDecoder<R>> {
        let mut r = r;
        try!(r.seek(SeekFrom::Start(0)));
        let head = &mut [0; HEADER_LEN as usize];
        try!(r.read_exact(head));
        if &head[..8] != "farbfeld".as_bytes() {
                return Err(FarbfeldError::FormatError("unexpected magic number".to_string()))
        }

        Ok(FarbfeldDecoder {
            r: r,
            width: BigEndian::read_u32(&head[8..]),
            height: BigEndian::read_u32(&head[12..]),
        })
    }

    /// Returns the `(width, height)` of the image.
    pub fn dimensions(&mut self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Returns the length in bytes for a row.
    pub fn row_len(&mut self) -> usize {
        self.width as usize * 8
    }

    /// Read a single row from the image and return the bytes read.
    /// # Failures
    /// Returns a `FarbfeldError::ImageEnd` if the `row` is greater as the `height`
    pub fn read_row(&mut self, row: u32,  buf: &mut [u8]) -> FarbfeldResult<usize> {
        if row > self.height { return Err(FarbfeldError::ImageEnd) }

        let row_len = self.row_len();
        let offset = HEADER_LEN + row as u64 * row_len as u64;
        try!(self.r.seek(SeekFrom::Start(offset)));
        try!(self.r.read_exact(&mut buf[..row_len]));
        Ok(row_len)
    }

    /// Read whole image into a `Vec<u8>`.
    pub fn read_image(&mut self) -> FarbfeldResult<Vec<u8>> {
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
    use FarbfeldDecoder;
    use tests::IMAGE_DATA;
    use FarbfeldError;

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
        try_panic!(img_data.write(&IMAGE_DATA[8..]));
        let buf = Cursor::new(img_data);

        match FarbfeldDecoder::new(buf) {
            Err(e) => match e {
                FarbfeldError::FormatError(_) => return,
                e => panic!("{:?}", e),
            },
            Ok(_) => panic!("Got Ok expected FarbfeldError::FormatError"),
        }
    }

    #[test]
    fn truncate_header() {
        let buf = Cursor::new(&IMAGE_DATA[0..8]);

        match FarbfeldDecoder::new(buf) {
            Err(FarbfeldError::IoError(e)) => {
                if e.kind() == ErrorKind::UnexpectedEof {
                    return
                } else { panic!("{:?}", e) }
            },
            Err(e) => panic!("{:?}", e),
            Ok(_) => panic!("Got Ok expected FarbfeldError::FormatError"),
        }

    }

    #[test]
    fn truncate_data() {
        let mut img_data = Vec::with_capacity(IMAGE_DATA.len()-1);
        try_panic!(img_data.write_all(&IMAGE_DATA[..IMAGE_DATA.len()-1]));
        let buf = Cursor::new(img_data);
        let mut img = FarbfeldDecoder::new(buf).unwrap();
        match img.read_image() {
            Err(FarbfeldError::IoError(e)) => {
                if e.kind() == ErrorKind::UnexpectedEof {
                    return
                } else { panic!("{:?}", e) }
            },
            Err(e) => panic!("{:?}", e),
            Ok(_) => panic!("Got Ok expected FarbfeldError::FormatError"),
        }
    }
}
