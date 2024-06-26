#[derive(Debug)]
pub(crate) enum Error {
    Io(std::io::Error),
    Ast(crate::compiler::ast::Error),
    Cst(crate::compiler::cst::Error),
    Rst(crate::compiler::rst::Error),
}

impl std::fmt::Display for Error {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Ast(e) => write!(f, "AST error: {}", e),
            Error::Cst(e) => write!(f, "CST error: {}", e),
            Error::Rst(e) => write!(f, "RST error: {}", e),
        }
    }
}

impl std::error::Error for Error {}
