use std::fmt::Display;
use crate::compiler::ast::parser::Rule;

#[derive(Debug)]
pub(crate) enum Error {
    Pest(Box<pest::error::Error<Rule>>),
    UnknownRule { rule_name: String },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Pest(e) => write!(f, "Parsing error: {}", e),
            Error::UnknownRule { rule_name } => write!(f, "Unknown rule: {}", rule_name),
        }
    }
}

impl std::error::Error for Error {}
