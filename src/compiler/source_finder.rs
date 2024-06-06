use crate::compiler::{CompilerSource, FileCompilerSource};
use std::path::PathBuf;

pub(crate) trait SourceFinder {
    fn find(&self, path: PathBuf) -> Option<impl CompilerSource>;
}

pub(crate) struct FileSourceFinder {
    root_dir: PathBuf,
}

impl SourceFinder for FileSourceFinder {
    fn find(&self, path: PathBuf) -> Option<impl CompilerSource> {
        let path = self.root_dir.join(path);

        let source = FileCompilerSource::new(path);
        Some(source)
    }
}

impl FileSourceFinder {
    pub(crate) fn new(root_dir: PathBuf) -> FileSourceFinder {
        FileSourceFinder { root_dir }
    }
}
