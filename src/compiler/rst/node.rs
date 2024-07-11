use crate::util::byte_buffer::ByteBuffer;

#[derive(Debug)]
pub(crate) struct HexoFile {
    emits: ByteBuffer,
}

impl HexoFile {
    pub(crate) fn new(emits: ByteBuffer) -> HexoFile {
        HexoFile {
            emits: emits,
        }
    }

    pub(crate) fn emits(&self) -> &ByteBuffer {
        &self.emits
    }
}
