use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) enum Error {
    UnknownCommand,
    CantCreateWatcher(notify::Error),
    CantStartWatcher(notify::Error),
    CantCrateOutputFile(std::io::Error),
    CantReadInputFile(std::io::Error),
    AstParsingFailed(crate::compiler::ast::Error),
    CompilationError(crate::compiler::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnknownCommand => write!(f, "Unknown command"),
            Error::CantCreateWatcher(e) => write!(f, "Can't create watcher: {}", e),
            Error::CantStartWatcher(e) => write!(f, "Can't start watcher: {}", e),
            Error::CantCrateOutputFile(e) => write!(f, "Can't create output file: {}", e),
            Error::CantReadInputFile(e) => write!(f, "Can't read input file: {}", e),
            Error::AstParsingFailed(e) => write!(f, "Ast parsing error: {}", e),
            Error::CompilationError(e) => write!(f, "Compilation error: {}", e),
        }
    }
}

impl std::error::Error for Error {}
