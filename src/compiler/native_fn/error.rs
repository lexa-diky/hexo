#[derive(Clone, Debug)]
pub(crate) enum Error {
    Unknown(String),
    MissingArgument { name: String, available_arguments: Vec<String>, function_name: String },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unknown(name) => write!(f, "Unknown error: {}", name),
            Error::MissingArgument { name, available_arguments, function_name } => {
                write!(f, "Missing argument {} for function {}. Available arguments: {:?}",
                    name, function_name, available_arguments
                )
            }
        }
    }
}

impl std::error::Error for Error {}
