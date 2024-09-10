use ext_php_rs::binary::Binary;
use ext_php_rs::binary_slice::BinarySlice;
use ext_php_rs::convert::IntoZval;
use ext_php_rs::error::Error;
use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;
use std::collections::HashMap;

#[php_class(name = "Takaram\\Psr7\\Internal\\VecStream")]
#[derive(Default, Debug)]
pub struct VecStream {
    body: Vec<u8>,
    pos: usize,
}

const SEEK_SET: i64 = 0;
const SEEK_CUR: i64 = 1;
const SEEK_END: i64 = 2;

impl VecStream {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_str(str: &str) -> Self {
        Self {
            body: Vec::from(str),
            pos: 0,
        }
    }
}

#[php_impl]
impl VecStream {
    pub fn from_string(str: BinarySlice<u8>) -> Self {
        Self {
            body: Vec::from(*str),
            pos: 0,
        }
    }

    pub fn __construct() -> Self {
        Self::new()
    }

    #[rename("__toString")]
    pub fn to_string(&self) -> Binary<u8> {
        self.body.clone().into()
    }

    pub fn close(&self) -> () {
        // Does not support close
    }

    pub fn detach(&self) -> Option<()> {
        // Does not support detach
        None
    }

    pub fn get_size(&self) -> Option<usize> {
        Some(self.body.len())
    }

    pub fn tell(&self) -> usize {
        self.pos
    }

    pub fn eof(&self) -> bool {
        self.body.len() <= self.pos
    }

    pub fn is_seekable(&self) -> bool {
        true
    }

    #[defaults(whence = 0)]
    pub fn seek(&mut self, offset: usize, whence: i64) -> () {
        let new_pos = match whence {
            SEEK_SET => offset,
            SEEK_CUR => self.pos + offset,
            SEEK_END => self.body.len() + offset,
            _ => return,
        };

        self.pos = new_pos;
    }

    pub fn rewind(&mut self) -> () {
        self.pos = 0;
    }

    pub fn is_writable(&self) -> bool {
        true
    }

    fn write(&mut self, string: BinarySlice<u8>) -> usize {
        let len = string.len();
        let new_pos = self.pos + len;
        if self.body.len() < new_pos {
            self.body.resize(new_pos, 0);
        }
        self.body[self.pos..new_pos].copy_from_slice(*string);
        self.pos = new_pos;

        len
    }

    pub fn is_readable(&self) -> bool {
        true
    }

    pub fn read(&mut self, length: usize) -> Binary<u8> {
        let start = self.pos;
        let mut end = start + length;
        if self.body.len() < end {
            end = self.body.len();
        }
        self.pos = end;

        self.body[start..end].to_vec().into()
    }

    pub fn get_contents(&mut self) -> Binary<u8> {
        let result = self.body[self.pos..].to_vec().into();
        self.pos = self.body.len();

        result
    }

    pub fn get_metadata(&self, key: Option<&str>) -> Result<Zval, Error> {
        if let Some(_key) = key {
            None::<String>.into_zval(false)
        } else {
            HashMap::<&str, Zval>::new().into_zval(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let stream = VecStream::from_str("foobar");
        assert_eq!(*stream.to_string(), "foobar".as_bytes());
    }

    #[test]
    fn get_size() {
        let stream = VecStream::from_str("foobar");
        assert_eq!(stream.get_size().unwrap(), 6);
    }

    #[test]
    fn write() {
        let mut stream = VecStream::from_str("foobar");
        let retval = stream.write("baz".as_bytes().into());
        assert_eq!(*stream.to_string(), "bazbar".as_bytes());
        assert_eq!(retval, 3);
        assert_eq!(stream.tell(), 3);
    }

    #[test]
    fn write_from_middle() {
        let mut stream = VecStream::from_str("foobar");
        stream.seek(2, SEEK_SET);
        let retval = stream.write("baz".as_bytes().into());
        assert_eq!(*stream.to_string(), "fobazr".as_bytes());
        assert_eq!(retval, 3);
        assert_eq!(stream.tell(), 5);
    }

    #[test]
    fn read_from_middle() {
        let mut stream = VecStream::from_str("foobar");
        stream.seek(2, SEEK_SET);
        assert_eq!(*stream.read(2), "ob".as_bytes());
        assert_eq!(*stream.read(1000), "ar".as_bytes());
    }

    #[test]
    fn get_contents_from_middle() {
        let mut stream = VecStream::from_str("foobar");
        stream.seek(2, SEEK_SET);
        assert_eq!(*stream.get_contents(), "obar".as_bytes());
        assert_eq!(stream.tell(), 6);
    }

    // #[test]
    // fn get_metadata_no_arg() {
    //     let stream = VecStream::from_str("foobar");
    //     let metadata = stream.get_metadata(None).unwrap();
    //     assert!(metadata.is_array());
    // }

    // #[test]
    // fn get_metadata_with_unknown_key() {
    //     let stream = VecStream::from_str("foobar");
    //     let metadata = stream.get_metadata(Some("foo")).unwrap();
    //     assert!(metadata.is_null());
    // }
}
