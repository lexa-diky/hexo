use std::path::PathBuf;
use clap::builder::Str;

pub(crate) trait CompilerSource {
    fn read(&self) -> String;

    fn path(&self) -> PathBuf;
}

pub(crate) struct StringCompilerSource {
    content: String,
    path: PathBuf,
}

impl StringCompilerSource {

    pub(crate) fn new(path: PathBuf, text: &str) -> StringCompilerSource {
        return StringCompilerSource {
            content: text.to_string(),
            path: path,
        };
    }
}

impl CompilerSource for StringCompilerSource {
    fn read(&self) -> String {
        return self.content.to_string();
    }

    fn path(&self) -> PathBuf {
        return self.path.clone();
    }
}