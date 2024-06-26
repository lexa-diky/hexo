use crate::compiler::rst::compilation_context::CompilationContext;
use hexo_io::byte_buffer::ByteBuffer;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct HexoFile {
    pub(crate) path: PathBuf,
    pub(crate) context: CompilationContext,
    pub(crate) emits: ByteBuffer,
}
