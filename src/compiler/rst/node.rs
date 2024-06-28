use crate::compiler::rst::compilation_context::CompilationContext;
use std::path::PathBuf;
use hexo_io::byte_buffer::ByteBuffer;

#[derive(Debug)]
pub(crate) struct HexoFile {
    pub(crate) path: PathBuf,
    pub(crate) context: CompilationContext,
    pub(crate) emits: ByteBuffer,
}
