use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) enum Error {
    UnknownCommand,
    FileWatcher(notify::Error),
    CantCrateOutputFile(std::io::Error),
    CantReadInputFile(std::io::Error),
    AstParsingFailed(crate::compiler::ast::Error),
    Compilation(crate::compiler::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnknownCommand => write!(f, "Unknown command"),
            Error::FileWatcher(e) => write!(f, "File watching error: {}", e),
            Error::CantCrateOutputFile(e) => write!(f, "Can't create output file: {}", e),
            Error::CantReadInputFile(e) => write!(f, "Can't read input file: {}", e),
            Error::AstParsingFailed(e) => write!(f, "Ast parsing error: {}", e),
            Error::Compilation(e) => write!(f, "Compilation error: {}", e),
        }
    }
}

impl std::error::Error for Error {}
