use crate::compiler::rst::scope::CompilationScope;
use crate::util::byte_buffer::ByteBuffer;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) struct HexoFile {
    path: PathBuf,
    context: CompilationScope,
    emits: ByteBuffer,
}

impl HexoFile {

    pub(crate) fn new(path: &Path, context: CompilationScope, emits: ByteBuffer) -> HexoFile {
        HexoFile {
            path: path.to_path_buf(),
            context,
            emits,
        }
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn context(&self) -> &CompilationScope {
        &self.context
    }

    pub(crate) fn emits(&self) -> &ByteBuffer {
        &self.emits
    }
}
