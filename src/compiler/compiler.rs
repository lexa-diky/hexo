use crate::ast::{AstParser, AstParserError};
use crate::compiler::{Compilation, CompilerSource, HexoCompilerContext};
use crate::cst::{CstParser, CstParserError};

#[derive(Debug)]
pub(crate) enum CompilerError {
    IO(std::io::Error),
    AST(AstParserError),
    CST(CstParserError),
}

pub(crate) struct HexoCompiler {
    context: HexoCompilerContext,
}

impl HexoCompiler {

    pub(crate) fn new(context: HexoCompilerContext) -> Self {
        HexoCompiler {
            context: context,
        }
    }

    pub(crate) fn compile<TSource: CompilerSource>(&self, source: TSource) -> Result<Compilation, CompilerError> {
        let ast_parser = AstParser::new();
        let cst_parser = CstParser::new();

        let source_text = source.read()
            .map_err(|e| CompilerError::IO(e))?;

        let ast = ast_parser.parse(source_text)
            .map_err(|e| CompilerError::AST(e))?;

        println!("{:#?}", ast);

        let cst = cst_parser.parse(source.path(), ast)
            .map_err(|e| CompilerError::CST(e))?;
        println!("{:#?}", cst);

        return Ok(Compilation::empty());
    }
}
