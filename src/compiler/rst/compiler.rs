use std::path::PathBuf;

use crate::compiler::cst::{CstAtom, CstAtomVec, CstEmitStatement, CstFile, CstFunctionStatement};
use crate::compiler::rst::compilation_context::{CompilationContext, ConstantBinding};
use crate::compiler::rst::node::HexoFile;
use crate::compiler::util::ByteBuffer;
use crate::compiler::HexoCompiler;

#[derive(Debug)]
pub(crate) enum RstCompilerError {
    UnresolvedConstant { name: String },
}

pub(crate) struct RstCompiler<'a> {
    parent: &'a HexoCompiler,
}

impl RstCompiler<'_> {
    pub(crate) fn new(parent: &HexoCompiler) -> RstCompiler {
        RstCompiler { parent: parent }
    }

    pub(crate) fn compile(&self, cst: &CstFile) -> Result<HexoFile, RstCompilerError> {
        let context = Self::build_context(&cst.path, &cst.main)?;

        let bb = Self::build_bytes(&context, &cst.main.emits)?;

        return Ok(HexoFile {
            path: cst.path.clone(),
            context: context,
            emits: bb,
        });
    }

    fn build_bytes(
        context: &CompilationContext,
        emits: &Vec<CstEmitStatement>,
    ) -> Result<ByteBuffer, RstCompilerError> {
        let mut byte_buffer = ByteBuffer::new();

        for emit in emits {
            Self::build_bytes_into(context, &emit.atoms, &mut byte_buffer)?
        }

        return Ok(byte_buffer);
    }

    fn build_bytes_into(
        context: &CompilationContext,
        atoms: &CstAtomVec,
        buffer: &mut ByteBuffer,
    ) -> Result<(), RstCompilerError> {
        for atom in atoms {
            match atom {
                CstAtom::Hex(byte) => buffer.push_byte(*byte),
                CstAtom::String(string) => buffer.push_string(string.clone()),
                CstAtom::Number(number) => buffer.push_u32_shrunk(*number),
                CstAtom::Constant { name } => Self::build_constant_into(context, &name, buffer)?,
                CstAtom::Function { .. } => {}
            }
        }

        Ok(())
    }

    fn build_constant_into(
        context: &CompilationContext,
        name: &String,
        buffer: &mut ByteBuffer,
    ) -> Result<(), RstCompilerError> {
        let constant_binding = context
            .get_constant(name)
            .ok_or(RstCompilerError::UnresolvedConstant { name: name.clone() })?;

        buffer.push_byte_buffer(&constant_binding.byte_buffer);

        Ok(())
    }

    fn build_context(
        file_path: &PathBuf,
        cst: &CstFunctionStatement,
    ) -> Result<CompilationContext, RstCompilerError> {
        let mut root_context = CompilationContext::new(file_path);

        Self::build_context_constants(&cst, &mut root_context)?;

        return Ok(root_context);
    }

    fn build_context_constants(
        cst: &&CstFunctionStatement,
        root_context: &mut CompilationContext,
    ) -> Result<(), RstCompilerError> {
        for constant in &cst.constants {
            let mut buff = ByteBuffer::new();
            Self::build_bytes_into(&root_context, &constant.atoms, &mut buff)?;
            root_context.bind_constant(ConstantBinding {
                name: constant.name.clone(),
                byte_buffer: buff,
            })
        }

        Ok(())
    }
}
