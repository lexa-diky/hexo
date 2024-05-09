use crate::compiler::cst::CstFile;
use crate::compiler::HexoCompiler;
use crate::compiler::rst::node::HexoFile;

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
        return todo!();
    }
}