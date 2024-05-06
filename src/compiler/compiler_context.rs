use std::path::PathBuf;

pub(crate) struct HexoCompilerContext {
    root_dir: PathBuf
}

impl HexoCompilerContext {

    pub(crate) fn new() -> Self {
        HexoCompilerContext {
            root_dir: PathBuf::from(".")
        }
    }
}
