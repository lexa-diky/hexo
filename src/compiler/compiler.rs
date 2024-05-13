use crate::compiler::ast::{AstNode, AstParser, AstParserError};
use crate::compiler::cst::{CstFile, CstParser, CstParserError};
use crate::compiler::rst::{HexoFile, RstCompiler, RstCompilerError};
use crate::compiler::{Compilation, CompilerSource, HexoCompilerContext};

#[derive(Debug)]
pub(crate) enum CompilerError {
    IO(std::io::Error),
    AST(AstParserError),
    CST(CstParserError),
    RST(RstCompilerError),
}

pub(crate) struct HexoCompiler {
    context: HexoCompilerContext,
}

impl HexoCompiler {
    pub(crate) fn new(context: HexoCompilerContext) -> Self {
        HexoCompiler { context: context }
    }

    pub(crate) fn compile_ast<TSource: CompilerSource>(
        &self,
        source: &TSource,
    ) -> Result<AstNode, CompilerError> {
        let ast_parser = AstParser::new();
        let source_text = source.read().map_err(|e| CompilerError::IO(e))?;

        return Ok(ast_parser
            .parse(source_text)
            .map_err(|e| CompilerError::AST(e))?);
    }

    pub(crate) fn compile_cst<TSource: CompilerSource>(
        &self,
        source: &TSource,
    ) -> Result<CstFile, CompilerError> {
        let ast = self.compile_ast(source)?;
        let cst_parser = CstParser::new();

        return Ok(cst_parser
            .parse(source.path(), ast)
            .map_err(|e| CompilerError::CST(e))?);
    }

    pub(crate) fn compile_rst<TSource: CompilerSource>(
        &self,
        source: &TSource,
    ) -> Result<HexoFile, CompilerError> {
        let cst = self.compile_cst(source)?;
        let rst_compiler = RstCompiler::new(self);

        return Ok(rst_compiler
            .compile(&cst)
            .map_err(|e| CompilerError::RST(e))?);
    }

    pub(crate) fn compile<TSource: CompilerSource>(
        &self,
        source: &TSource,
    ) -> Result<Compilation, CompilerError> {
        let cst = self.compile_cst(source)?;
        let rst_compiler = RstCompiler::new(self);
        let rst = rst_compiler
            .compile(&cst)
            .map_err(|e| CompilerError::RST(e))?;

        return Ok(Compilation::from(rst.emits.as_vec()));
    }
}
