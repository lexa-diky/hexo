use std::path::PathBuf;
use crate::compiler::{CompilerSource, FileCompilerSource, StringCompilerSource};

pub(crate) trait SourceFinder {

    fn find(&self, path: PathBuf) -> Option<impl CompilerSource>;
}

struct FileSourceFinder {
    root_dir: PathBuf
}

impl SourceFinder for FileSourceFinder {

    fn find(&self, path: PathBuf) -> Option<impl CompilerSource> {
        let path = self.root_dir.join(path);

        let source = FileCompilerSource::new(path);
        Some(
            source
        )
    }
}