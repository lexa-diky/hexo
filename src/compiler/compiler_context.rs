pub(crate) struct HexoCompilerContext {
    safe_mode: bool
}

impl HexoCompilerContext {
    pub(crate) fn new(safe_mode: bool) -> Self {
        HexoCompilerContext {
            safe_mode
        }
    }

    pub(crate) fn safe_mode(&self) -> bool {
        self.safe_mode
    }
}
