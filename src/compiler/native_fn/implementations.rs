use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::compiler::native_fn::signature::{NativeFunction, NativeFunctionSignature};
use crate::compiler::native_fn::NativeFunctionError;
use crate::compiler::util::ByteBuffer;

pub(crate) fn create_len_native_function() -> NativeFunction {
    NativeFunction {
        signature: NativeFunctionSignature {
            name: String::from("len"),
        },
        executor: |arguments: HashMap<String, ByteBuffer>| {
            let mut result = ByteBuffer::new();
            let arg0 = get_argument_at(&arguments, 0)?;

            let len = arg0.len() as u32;
            result.push_u32_shrunk(len);
            Ok(result)
        },
    }
}

pub(crate) fn create_pad_left_native_function() -> NativeFunction {
    NativeFunction {
        signature: NativeFunctionSignature {
            name: String::from("pad_left"),
        },
        executor: |arguments: HashMap<String, ByteBuffer>| {
            let mut arg0 = get_argument_at(&arguments, 0)?;
            let arg1 = get_argument_at(&arguments, 1)?;

            arg0.pad_left(arg1.as_usize());

            Ok(arg0)
        },
    }
}

pub(crate) fn create_pad_right_native_function() -> NativeFunction {
    NativeFunction {
        signature: NativeFunctionSignature {
            name: String::from("pad_right"),
        },
        executor: |arguments: HashMap<String, ByteBuffer>| {
            let mut arg0 = get_argument_at(&arguments, 0)?;
            let arg1 = get_argument_at(&arguments, 1)?;

            arg0.pad_right(arg1.as_usize());

            Ok(arg0)
        },
    }
}

pub(crate) fn create_cmd_native_function() -> NativeFunction {
    NativeFunction {
        signature: NativeFunctionSignature {
            name: String::from("cmd"),
        },
        executor: |arguments: HashMap<String, ByteBuffer>| {
            let command = get_argument_at(&arguments, 0)?
                .as_string()
                .map_err(|e| NativeFunctionError::Unknown(e.to_string()))?;

            let output = std::process::Command::new(command)
                .output()
                .map_err(|e|
                    NativeFunctionError::Unknown(
                        format!("Error executing command: {}", e)
                    )
                )?;

            let buffer = ByteBuffer::from(output.stdout);

            Ok(buffer)
        },
    }
}

pub(crate) fn create_read_file_native_function() -> NativeFunction {
    return NativeFunction {
        signature: NativeFunctionSignature {
            name: String::from("read_file"),
        },
        executor: |arguments: HashMap<String, ByteBuffer>| {
            let arg0 = get_argument_at(&arguments, 0)?;

            let file_path = arg0.as_string()
                .map_err(|e| NativeFunctionError::Unknown(e.to_string()))?;

            let mut file = File::open(file_path)
                .map_err(|e|
                    NativeFunctionError::Unknown(
                        format!("Error executing command: {}", e)
                    )
                )?;

            let mut buf_string = String::new();
            file.read_to_string(&mut buf_string)
                .map_err(|e|
                    NativeFunctionError::Unknown(
                        format!("Error executing command: {}", e)
                    )
                )?;

            let buffer = ByteBuffer::from(buf_string.as_bytes().to_vec());

            Ok(buffer)
        },
    };
}

fn get_argument_at(arguments: &HashMap<String, ByteBuffer>, pos: usize) -> Result<ByteBuffer, NativeFunctionError> {
    Ok(
        arguments
            .get(&pos.to_string())
            .ok_or(NativeFunctionError::MissingArgument { name: pos.to_string() })?
            .clone()
    )
}
