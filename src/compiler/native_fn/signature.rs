use std::collections::HashMap;
use hexo_io::byte_buffer::ByteBuffer;
use crate::compiler::native_fn::error::Error;

#[derive(Clone, Debug)]
pub(crate) struct NativeFunctionSignature {
    pub name: String,
}

type NativeFunctionExecutor =
    fn(HashMap<String, ByteBuffer>) -> Result<ByteBuffer, Error>;

#[derive(Clone, Debug)]
pub(crate) struct NativeFunction {
    pub(crate) signature: NativeFunctionSignature,
    pub(crate) executor: NativeFunctionExecutor,
}
