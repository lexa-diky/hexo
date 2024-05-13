use crate::compiler::rst::compilation_context::CompilationContext;
use crate::compiler::util::ByteBuffer;
use std::path::PathBuf;

pub(crate) struct HexoFile {
    pub(crate) path: PathBuf,
    pub(crate) context: CompilationContext,
    pub(crate) emits: ByteBuffer,
}
