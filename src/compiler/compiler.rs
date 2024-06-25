use crate::compiler::ast::{AstNode, AstParser};
use crate::compiler::cst::{CstFile, CstParser};
use crate::compiler::rst::{HexoFile, RstCompiler, RstCompilerError};
use crate::compiler::{Compilation, CompilerSource, HexoCompilerContext};

#[derive(Debug)]
pub(crate) enum CompilerError {
    IO(std::io::Error),
    AST(crate::compiler::ast::Error),
    CST(crate::compiler::cst::Error),
    RST(RstCompilerError),
}

pub(crate) struct HexoCompiler {
    context: HexoCompilerContext,
}

impl HexoCompiler {
    pub(crate) fn new(context: HexoCompilerContext) -> Self {
        HexoCompiler { context }
    }

    pub(crate) fn compile_ast<TSource: CompilerSource>(
        &self,
        source: &TSource,
    ) -> Result<AstNode, CompilerError> {
        let ast_parser = AstParser::new();
        let source_text = source.read().map_err(CompilerError::IO)?;

        ast_parser
            .parse(source_text)
            .map_err(CompilerError::AST)
    }

    pub(crate) fn compile_cst<TSource: CompilerSource>(
        &self,
        source: &TSource,
    ) -> Result<CstFile, CompilerError> {
        let ast = self.compile_ast(source)?;
        let cst_parser = CstParser::new();

        cst_parser
            .parse(source.path(), ast)
            .map_err(CompilerError::CST)
    }

    pub(crate) fn compile_rst<TSource: CompilerSource>(
        &self,
        source: &TSource,
    ) -> Result<HexoFile, CompilerError> {
        let cst = self.compile_cst(source)?;
        let rst_compiler = RstCompiler::new(self);

        rst_compiler
            .compile(&cst)
            .map_err(CompilerError::RST)
    }

    pub(crate) fn compile<TSource: CompilerSource>(
        &self,
        source: &TSource,
    ) -> Result<Compilation, CompilerError> {
        let rst = self.compile_rst(source)?;

        Ok(Compilation::from(rst.emits.as_vec()))
    }
}
