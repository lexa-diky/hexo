use crate::compiler::native_fn::error::Error;
use crate::util::byte_buffer::ByteBuffer;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub(crate) struct NativeFunctionSignature {
    pub name: String,
}

type NativeFunctionExecutor = fn(HashMap<String, ByteBuffer>) -> Result<ByteBuffer, Error>;

#[derive(Clone, Debug)]
pub(crate) struct NativeFunction {
    pub(crate) signature: NativeFunctionSignature,
    pub(crate) executor: NativeFunctionExecutor,
}
