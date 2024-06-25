use std::fmt::{Display, Formatter};
use crate::compiler::ast::Error;

#[derive(Debug)]
pub(crate) enum CliError {
    UnknownCommand,
    CantCreateWatcher(notify::Error),
    CantStartWatcher(notify::Error),
    CantCrateOutputFile(std::io::Error),
    CantReadInputFile(std::io::Error),
    AstParsingFailed(Error),
    CompilationError(crate::compiler::CompilerError),
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::UnknownCommand => write!(f, "Unknown command"),
            CliError::CantCreateWatcher(e) => write!(f, "Can't create watcher: {}", e),
            CliError::CantStartWatcher(e) => write!(f, "Can't start watcher: {}", e),
            CliError::CantCrateOutputFile(e) => write!(f, "Can't create output file: {}", e),
            CliError::CantReadInputFile(e) => write!(f, "Can't read input file: {}", e),
            CliError::AstParsingFailed(e) => write!(f, "Ast parsing error: {}", e),
            CliError::CompilationError(e) => write!(f, "Compilation error: {}", e),
        }
    }
}

impl std::error::Error for CliError {}
