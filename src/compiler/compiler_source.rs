use std::fs::File;
use std::io::Read;
use std::path::{PathBuf};

pub(crate) trait CompilerSource {
    fn read(&self) -> Result<String, std::io::Error>;

    fn path(&self) -> PathBuf;
}

pub(crate) struct StringCompilerSource {
    content: String,
    path: PathBuf,
}

impl StringCompilerSource {
    pub(crate) fn new(path: PathBuf, text: &str) -> StringCompilerSource {
        StringCompilerSource {
            content: text.to_string(),
            path,
        }
    }
}

impl CompilerSource for StringCompilerSource {
    fn read(&self) -> Result<String, std::io::Error> {
        Ok(self.content.clone())
    }

    fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

pub(crate) struct FileCompilerSource {
    path: PathBuf,
}

impl FileCompilerSource {
    pub(crate) fn new(path: PathBuf) -> FileCompilerSource {
        FileCompilerSource { path }
    }
}

impl CompilerSource for FileCompilerSource {
    fn read(&self) -> Result<String, std::io::Error> {
        let mut p = File::open(self.path.clone())?;
        let mut buff = String::new();
        p.read_to_string(&mut buff)?;

        Ok(buff)
    }

    fn path(&self) -> PathBuf {
        self.path.clone()
    }
}
