use hexo_ast::{AstNode, AstParser};
use hexo_cst::{CstFile, CstParser};
use crate::compiler::error::Error;
use crate::compiler::rst::{HexoFile, RstCompiler};
use crate::compiler::{Compilation, CompilerSource, HexoCompilerContext};

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
    ) -> Result<AstNode, Error> {
        let ast_parser = AstParser::default();
        let source_text = source.read().map_err(Error::Io)?;

        ast_parser.parse(source_text.as_str()).map_err(Error::Ast)
    }

    pub(crate) fn compile_cst<TSource: CompilerSource>(
        &self,
        source: &TSource,
    ) -> Result<CstFile, Error> {
        let ast = self.compile_ast(source)?;
        let cst_parser = CstParser::default();

        cst_parser.parse(source.path(), ast).map_err(Error::Cst)
    }

    pub(crate) fn compile_rst<TSource: CompilerSource>(
        &self,
        source: &TSource,
    ) -> Result<HexoFile, Error> {
        let cst = self.compile_cst(source)?;
        let rst_compiler = RstCompiler::new(self);

        rst_compiler.compile(&cst).map_err(Error::Rst)
    }

    pub(crate) fn compile<TSource: CompilerSource>(
        &self,
        source: &TSource,
    ) -> Result<Compilation, Error> {
        let rst = self.compile_rst(source)?;

        Ok(Compilation::from(rst.emits.to_vec()))
    }
}
