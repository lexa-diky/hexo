use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use crate::util::id::HexoId;
use crate::util::logger;

pub(crate) trait CompilerSource {
    fn read(&self) -> Result<String, std::io::Error>;

    fn path(&self) -> &Path;
}

pub(crate) struct FileCompilerSource {
    path: PathBuf,
}

impl FileCompilerSource {
    pub(crate) fn new(path: &Path) -> FileCompilerSource {
        FileCompilerSource { path: path.to_path_buf() }
    }
}

impl CompilerSource for FileCompilerSource {
    fn read(&self) -> Result<String, std::io::Error> {
        logger::debug!("Reading source file: {:?}", self.path);
        let mut p = File::open(self.path.clone())?;
        let mut buff = String::new();
        p.read_to_string(&mut buff)?;

        Ok(buff)
    }

    fn path(&self) -> &Path {
        self.path.as_path()
    }
}

pub(crate) struct LiteralCompilerSource {
    content: String,
    path: PathBuf,
}

impl CompilerSource for LiteralCompilerSource {
    fn read(&self) -> Result<String, std::io::Error> {
        Ok(self.content.clone())
    }

    fn path(&self) -> &Path {
        self.path.as_path()
    }
}

impl LiteralCompilerSource {
    pub(crate) fn anonymous(content: String) -> LiteralCompilerSource {
        LiteralCompilerSource {
            content: content,
            path: Path::new(
                format!("hexo://anonymous/{}", HexoId::next()).as_str()
            ).to_path_buf(),
        }
    }
}


#[cfg(test)]
pub(crate) mod tests {
    use std::fs::File;
    use std::io::Read;
    use std::path::{Path, PathBuf};
    use crate::compiler::CompilerSource;

    pub(crate) struct EagerCompilerSource {
        content: String,
        path: PathBuf,
    }

    impl CompilerSource for EagerCompilerSource {
        fn read(&self) -> Result<String, std::io::Error> {
            Ok(self.content.clone())
        }

        fn path(&self) -> &Path {
            return self.path.as_path();
        }
    }

    impl EagerCompilerSource {
        pub(crate) fn new<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
            let pat_ref = path.as_ref();
            let mut source_file = File::open(pat_ref)?;
            let mut content = String::new();
            source_file.read_to_string(&mut content)?;

            Ok(
                EagerCompilerSource { content: content, path: pat_ref.to_path_buf() }
            )
        }
    }
}