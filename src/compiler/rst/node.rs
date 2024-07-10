use crate::compiler::rst::compilation_context::CompilationContext;
use crate::util::byte_buffer::ByteBuffer;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) struct HexoFile {
    path: PathBuf,
    context: CompilationContext,
    emits: ByteBuffer,
}

impl HexoFile {

    pub(crate) fn new(path: &Path, context: CompilationContext, emits: ByteBuffer) -> HexoFile {
        HexoFile {
            path: path.to_path_buf(),
            context,
            emits,
        }
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn context(&self) -> &CompilationContext {
        &self.context
    }

    pub(crate) fn emits(&self) -> &ByteBuffer {
        &self.emits
    }
}
