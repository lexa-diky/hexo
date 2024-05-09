use std::path::PathBuf;
use crate::compiler::cst::{CstEmitStatement, CstFile, CstFunctionStatement};
use crate::compiler::HexoCompiler;
use crate::compiler::rst::compilation_context::CompilationContext;
use crate::compiler::rst::node::HexoFile;
use crate::compiler::util::ByteBuffer;
use crate::cst_legacy::CstAtom;

#[derive(Debug)]
pub(crate) enum RstCompilerError {}

pub(crate) struct RstCompiler<'a> {
    parent: &'a HexoCompiler,
}

impl RstCompiler<'_> {

    pub(crate) fn new(parent: &HexoCompiler) -> RstCompiler {
        RstCompiler {
            parent: parent
        }
    }

    pub(crate) fn compile(&self, cst: &CstFile) -> Result<HexoFile, RstCompilerError> {
        let context = Self::build_context(&cst.path, &cst.main)?;

        let bb = Self::build_bytes(&context, &cst.main.emits)?;

        return Ok(
            HexoFile {
                path: cst.path.clone(),
                context: context,
                emits: bb,
            }
        );
    }

    fn build_bytes(context: &CompilationContext, emits: &Vec<CstEmitStatement>) -> Result<ByteBuffer, RstCompilerError> {
        let byte_buffer = ByteBuffer::new();

        return Ok(byte_buffer);
    }

    fn build_context(file_path: &PathBuf, cst: &CstFunctionStatement) -> Result<CompilationContext, RstCompilerError> {
        let root_context = CompilationContext::new(file_path);

        return Ok(root_context);
    }
}

