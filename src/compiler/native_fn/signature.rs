use crate::compiler::native_fn::error::Error;
use crate::util::byte_buffer::ByteBuffer;
use std::collections::HashMap;
use crate::compiler::HexoCompiler;

#[derive(Clone, Debug)]
pub(crate) struct NativeFunctionSignature {
    pub(crate) name: String,
}

impl NativeFunctionSignature {
    pub(crate) fn new(name: &str) -> NativeFunctionSignature {
        NativeFunctionSignature {
            name: String::from(name),
        }
    }
}

type NativeFunctionExecutor = fn(HashMap<String, ByteBuffer>, &HexoCompiler) -> Result<ByteBuffer, Error>;

#[derive(Clone, Debug)]
pub(crate) struct NativeFunction {
    pub(crate) signature: NativeFunctionSignature,
    pub(crate) executor: NativeFunctionExecutor,
}
