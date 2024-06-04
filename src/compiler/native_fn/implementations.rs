use std::collections::HashMap;

use crate::compiler::native_fn::signature::{NativeFunction, NativeFunctionSignature};
use crate::compiler::native_fn::NativeFunctionError;
use crate::compiler::util::ByteBuffer;

pub(crate) fn create_len_native_function() -> NativeFunction {
    return NativeFunction {
        signature: NativeFunctionSignature {
            name: String::from("len"),
        },
        executor: |arguments: HashMap<String, ByteBuffer>| {
            let mut result = ByteBuffer::new();
            let arg0 = arguments
                .get("0")
                .ok_or(NativeFunctionError::Unknown("0".to_string()))?;
            let len = arg0.len() as u32;
            result.push_u32_shrunk(len);
            return Ok(result);
        },
    };
}
