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

type NativeFunctionExecutor = fn(&HashMap<String, ByteBuffer>, &HexoCompiler) -> Result<ByteBuffer, Error>;

#[derive(Clone, Debug)]
pub(crate) struct NativeFunction {
    signature: NativeFunctionSignature,
    executor: NativeFunctionExecutor,
}

impl NativeFunction {
    pub(crate) fn new(signature: NativeFunctionSignature, executor: NativeFunctionExecutor) -> NativeFunction {
        NativeFunction {
            signature: signature,
            executor: executor,
        }
    }

    pub(crate) fn signature(&self) -> &NativeFunctionSignature {
        &self.signature
    }

    pub(crate) fn executor(&self) -> NativeFunctionExecutor {
        self.executor
    }
}