use crate::compiler::util::ByteBuffer;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub(crate) enum NativeFunctionError {
    Unknown(String),
    MissingArgument { name: String, available_arguments: Vec<String>, function_name: String },
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
