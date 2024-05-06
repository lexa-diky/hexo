pub(crate) struct CompilationResult {
    pub(crate) content: Vec<u8>
}

impl CompilationResult {

    pub(crate) fn empty() -> Self {
        CompilationResult {
            content: Vec::new()
        }
    }
}