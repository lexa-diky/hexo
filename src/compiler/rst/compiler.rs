use std::path::PathBuf;
use crate::compiler::cst::{CstAtom, CstEmitStatement, CstFile, CstFunctionStatement};
use crate::compiler::HexoCompiler;
use crate::compiler::rst::compilation_context::CompilationContext;
use crate::compiler::rst::node::HexoFile;
use crate::compiler::util::ByteBuffer;

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
        let mut byte_buffer = ByteBuffer::new();

        for emit in emits {
            Self::build_bytes_into(context, emit, &mut byte_buffer)
        }

        return Ok(byte_buffer);
    }

    fn build_bytes_into(context: &CompilationContext, statement: &CstEmitStatement, buffer: &mut ByteBuffer) {
        for atom in &statement.atoms {
            match atom {
                CstAtom::Hex(byte) => buffer.push_byte(*byte),
                CstAtom::String(string) => buffer.push_string(string.clone()),
                CstAtom::Number(number) => buffer.push_shrunk_u32(*number),
                CstAtom::Constant { .. } => {}
                CstAtom::Function { .. } => {}
            }
        }
    }

    fn build_context(file_path: &PathBuf, cst: &CstFunctionStatement) -> Result<CompilationContext, RstCompilerError> {
        let root_context = CompilationContext::new(file_path);

        return Ok(root_context);
    }
}

