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

pub(crate) fn create_pad_left_native_function() -> NativeFunction {
    return NativeFunction {
        signature: NativeFunctionSignature {
            name: String::from("pad_left"),
        },
        executor: |arguments: HashMap<String, ByteBuffer>| {
            let mut arg0 = arguments
                .get("0")
                .ok_or(NativeFunctionError::Unknown("0".to_string()))?
                .clone();

            let mut arg1 = arguments
                .get("1")
                .ok_or(NativeFunctionError::Unknown("1".to_string()))?
                .clone();

            arg0.pad_left(arg1.as_usize());

            return Ok(arg0);
        },
    };
}

pub(crate) fn create_pad_right_native_function() -> NativeFunction {
    return NativeFunction {
        signature: NativeFunctionSignature {
            name: String::from("pad_right"),
        },
        executor: |arguments: HashMap<String, ByteBuffer>| {
            let mut arg0 = arguments
                .get("0")
                .ok_or(NativeFunctionError::Unknown("0".to_string()))?
                .clone();

            let mut arg1 = arguments
                .get("1")
                .ok_or(NativeFunctionError::Unknown("1".to_string()))?
                .clone();

            arg0.pad_right(arg1.as_usize());

            return Ok(arg0);
        },
    };
}