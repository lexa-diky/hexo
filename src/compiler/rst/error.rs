#[derive(Debug)]
pub(crate) enum Error {
    UnresolvedConstant { name: String },
    UnresolvedFunction { name: String },
    NativeFunctionExecution(crate::compiler::native_fn::Error),
    NativeFunctionIsUnsafe { name: String },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnresolvedConstant { name } => {
                write!(f, "Unresolved constant: {}", name)
            }
            Error::UnresolvedFunction { name } => {
                write!(f, "Unresolved function: {}", name)
            }
            Error::NativeFunctionExecution(e) => {
                write!(f, "Native function execution error: {}", e)
            }
            Error::NativeFunctionIsUnsafe { name } => {
                write!(f, "Native function is unsafe: {}", name)
            }
        }
    }
}

impl std::error::Error for Error {}
