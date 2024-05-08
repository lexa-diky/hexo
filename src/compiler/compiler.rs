use crate::ast::AstParser;
use crate::compiler::{Compilation, CompilerSource, HexoCompilerContext};

pub(crate) enum CompilerError {
    IO(std::io::Error),
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
        let source_text = source.read()
            .map_err(|e| CompilerError::IO(e))?;

        return Ok(Compilation::empty());
    }
}
