use std::collections::HashMap;
use std::fmt::format;
use std::fs::File;
use std::io::Read;

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

pub(crate) fn create_cmd_native_function() -> NativeFunction {
    return NativeFunction {
        signature: NativeFunctionSignature {
            name: String::from("cmd"),
        },
        executor: |arguments: HashMap<String, ByteBuffer>| {
            let mut arg0 = arguments
                .get("0")
                .ok_or(NativeFunctionError::Unknown("0".to_string()))?
                .clone();


            let command = arg0.as_string();
            let output = std::process::Command::new(command)
                .output()
                .map_err(|e|
                    NativeFunctionError::Unknown(
                        format!("Error executing command: {}", e.to_string())
                    )
                )?;

            let buffer = ByteBuffer::from(output.stdout);

            return Ok(buffer);
        },
    };
}

pub(crate) fn create_read_file_native_function() -> NativeFunction {
    return NativeFunction {
        signature: NativeFunctionSignature {
            name: String::from("read_file"),
        },
        executor: |arguments: HashMap<String, ByteBuffer>| {
            let mut arg0 = arguments
                .get("0")
                .ok_or(NativeFunctionError::Unknown("0".to_string()))?
                .clone();


            let file_path = arg0.as_string();

            let mut file = File::open(file_path)
                .map_err(|e|
                    NativeFunctionError::Unknown(
                        format!("Error executing command: {}", e.to_string())
                    )
                )?;

            let mut buf_string = String::new();
            file.read_to_string(&mut buf_string)
                .map_err(|e|
                    NativeFunctionError::Unknown(
                        format!("Error executing command: {}", e.to_string())
                    )
                )?;

            let buffer = ByteBuffer::from(buf_string.as_bytes().to_vec());

            return Ok(buffer);
        },
    };
}
