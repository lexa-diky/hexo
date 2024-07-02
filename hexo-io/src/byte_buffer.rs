use std::fmt::{Debug, Formatter};
use std::string::FromUtf8Error;

#[derive(Clone, Default)]
pub struct ByteBuffer {
    inner: Vec<u8>,
}

impl Debug for ByteBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self.inner))
    }
}

impl ByteBuffer {

    pub fn push_byte(&mut self, byte: u8) {
        self.inner.push(byte);
    }

    pub fn push_string(&mut self, string: String) {
        self.inner.extend_from_slice(string.as_bytes());
    }

    pub fn push_u32_shrunk(&mut self, num: u32) {
        self.inner.extend(Self::_to_shrunk_bytes(num));
    }

    pub fn push_byte_buffer(&mut self, other: &ByteBuffer) {
        self.inner.extend(other.to_vec());
    }

    /// Moves the byte buffer to the left by the specified [size]
    pub fn pad_left(&mut self, size: usize) {
        let padding = size.checked_sub(self.inner.len());
        if let Some(padding) = padding {
            if padding > 0 {
                let mut padding_vec = vec![0; padding];
                padding_vec.append(&mut self.inner);
                self.inner = padding_vec;
            }
        }
    }

    /// Moves the byte buffer to the right by the specified [size]
    pub fn pad_right(&mut self, size: usize) {
        let padding = size.checked_sub(self.inner.len());

        if let Some(padding) = padding {
            if padding > 0 {
                let mut padding_vec = vec![0; padding];
                self.inner.append(padding_vec.as_mut());
            }
        }
    }

    /// Returns the length of the byte buffer
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn as_usize_unsafe(&self) -> usize {
        let mut padded = self.clone();
        padded.pad_left(4);
        ((padded.inner[0] as usize) << 24)
            + ((padded.inner[1] as usize) << 16)
            + ((padded.inner[2] as usize) << 8)
            + (padded.inner[3] as usize)
    }

    /// Clones inner representation of the byte buffer and returns Vec<u8> of it
    pub fn to_vec(&self) -> Vec<u8> {
        self.inner.clone()
    }

    pub fn to_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.inner.clone())
    }

    fn _to_shrunk_bytes(value: u32) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut value = value;
        while value > 0 {
            bytes.push((value & 0xFF) as u8);
            value >>= 8;
        }
        bytes
    }
}

impl From<Vec<u8>> for ByteBuffer {
    fn from(value: Vec<u8>) -> Self {
        ByteBuffer { inner: value }
    }
}

mod test {
    use crate::byte_buffer::ByteBuffer;

    #[test]
    fn byte_push() {
        let mut buffer = ByteBuffer::new();
        buffer.push_byte(0x01);
        buffer.push_byte(0x02);

        assert_eq!(buffer.len(), 2);

        assert_eq!(buffer.to_vec(), vec![0x01, 0x02]);
    }

    #[test]
    fn string_push() {
        let mut buffer = ByteBuffer::new();
        buffer.push_string("hello world".to_string());

        assert_eq!(buffer.len(), 11);

        assert_eq!(
            buffer.to_vec(),
            vec![104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]
        );
    }

    #[test]
    fn u32_push() {
        let mut buffer = ByteBuffer::new();
        buffer.push_u32_shrunk(13);

        assert_eq!(buffer.len(), 1);

        assert_eq!(buffer.to_vec(), vec![13]);
    }
}
