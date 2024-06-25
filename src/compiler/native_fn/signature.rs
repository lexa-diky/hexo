use crate::compiler::util::ByteBuffer;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub(crate) enum NativeFunctionError {
    Unknown(String),
    MissingArgument { name: String, available_arguments: Vec<String>, function_name: String },
}

impl std::fmt::Display for NativeFunctionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NativeFunctionError::Unknown(name) => write!(f, "Unknown error: {}", name),
            NativeFunctionError::MissingArgument { name, available_arguments, function_name } => {
                write!(f, "Missing argument {} for function {}. Available arguments: {:?}",
                    name, function_name, available_arguments
                )
            }
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct NativeFunctionSignature {
    pub name: String,
}

type NativeFunctionExecutor =
    fn(HashMap<String, ByteBuffer>) -> Result<ByteBuffer, NativeFunctionError>;

#[derive(Clone, Debug)]
pub(crate) struct NativeFunction {
    pub(crate) signature: NativeFunctionSignature,
    pub(crate) executor: NativeFunctionExecutor,
}
