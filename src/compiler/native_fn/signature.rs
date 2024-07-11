use crate::compiler::native_fn::error::Error;
use crate::util::byte_buffer::ByteBuffer;
use std::collections::HashMap;
use crate::compiler::HexoCompiler;
use crate::compiler::native_fn::arguments::NativeFunctionArguments;

#[derive(Clone, Debug)]
pub(crate) struct NativeFunctionSignature {
    name: String,
    is_safe: bool
}

impl NativeFunctionSignature {
    pub(crate) fn new(name: &str) -> NativeFunctionSignature {
        NativeFunctionSignature {
            name: String::from(name),
            is_safe: true
        }
    }

    pub(crate) fn new_unsafe(name: &str) -> NativeFunctionSignature {
        NativeFunctionSignature {
            name: String::from(name),
            is_safe: false
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn is_safe(&self) -> bool {
        self.is_safe
    }
}

type NativeFunctionExecutor = fn(NativeFunctionArguments, &HexoCompiler) -> Result<ByteBuffer, Error>;

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

pub(crate) trait NativeFunctionDefinition {
    fn create(&self) -> NativeFunction;
}
