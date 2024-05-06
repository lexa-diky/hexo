use crate::compiler::{CompilationResult, CompilerSource, HexoCompilerContext};

pub(crate) struct HexoCompiler {

}

impl HexoCompiler {

    pub(crate) fn new(context: HexoCompilerContext) -> Self {
        HexoCompiler {}
    }

    pub(crate) fn compile<TSource: CompilerSource>(&self, source: TSource) -> CompilationResult {
        return CompilationResult::empty();
    }
}
