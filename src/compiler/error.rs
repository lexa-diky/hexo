#[derive(Debug)]
pub(crate) enum Error {
    IO(std::io::Error),
    AST(crate::compiler::ast::Error),
    CST(crate::compiler::cst::Error),
    RST(crate::compiler::rst::Error),
}

impl std::fmt::Display for Error {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(e) => write!(f, "IO error: {}", e),
            Error::AST(e) => write!(f, "AST error: {}", e),
            Error::CST(e) => write!(f, "CST error: {}", e),
            Error::RST(e) => write!(f, "RST error: {}", e),
        }
    }
}

impl std::error::Error for Error {}
