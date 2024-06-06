use crate::compiler::util::encoding::to_shrunk_bytes;
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub(crate) struct ByteBuffer {
    inner: Vec<u8>,
}

impl ByteBuffer {
    pub(crate) fn new() -> Self {
        ByteBuffer { inner: Vec::new() }
    }

    pub(crate) fn push_byte(&mut self, byte: u8) {
        self.inner.push(byte);
    }

    pub(crate) fn push_string(&mut self, string: String) {
        self.inner.extend_from_slice(string.as_bytes());
    }

    pub(crate) fn push_u32_shrunk(&mut self, num: u32) {
        self.inner.extend(to_shrunk_bytes(num));
    }

    pub(crate) fn push_byte_buffer(&mut self, other: &ByteBuffer) {
        self.inner.extend(other.as_vec());
    }

    pub(crate) fn pad_left(&mut self, size: usize) {
        let padding = size - self.inner.len();

        if padding > 0 {
            let mut padding_vec = vec![0; padding];
            padding_vec.append(&mut self.inner);
            self.inner = padding_vec;
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.inner.len()
    }

    pub(crate) fn as_vec(&self) -> Vec<u8> {
        self.inner.clone()
    }

    pub(crate) fn as_usize(&self) -> usize {
        let mut padded = self.clone();
        padded.pad_left(4);
        return ((padded.inner[0] as usize) << 24) +
            ((padded.inner[1] as usize) << 16) +
            ((padded.inner[2] as usize) <<  8) +
            ((padded.inner[3] as usize) <<  0);
    }
}

mod test {
    use crate::compiler::util::ByteBuffer;

    #[test]
    fn byte_push() {
        let mut buffer = ByteBuffer::new();
        buffer.push_byte(0x01);
        buffer.push_byte(0x02);

        assert_eq!(buffer.len(), 2);

        assert_eq!(buffer.as_vec(), vec![0x01, 0x02]);
    }

    #[test]
    fn string_push() {
        let mut buffer = ByteBuffer::new();
        buffer.push_string("hello world".to_string());

        assert_eq!(buffer.len(), 11);

        assert_eq!(
            buffer.as_vec(),
            vec![104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]
        );
    }

    #[test]
    fn u32_push() {
        let mut buffer = ByteBuffer::new();
        buffer.push_u32_shrunk(13);

        assert_eq!(buffer.len(), 1);

        assert_eq!(buffer.as_vec(), vec![13]);
    }
}

impl Debug for ByteBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self.inner))
    }
}
