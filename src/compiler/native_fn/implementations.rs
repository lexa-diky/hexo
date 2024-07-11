use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::compiler::native_fn::error::Error;
use crate::compiler::native_fn::signature::{NativeFunction, NativeFunctionSignature};
use crate::util::byte_buffer::ByteBuffer;

pub(crate) fn create_len_native_function() -> NativeFunction {
    NativeFunction::new(
        NativeFunctionSignature::new("len"),
        |arguments, _| {
            let mut result = ByteBuffer::default();
            let arg0 = arguments.get_named_argument("utf8")
                .unwrap_or_else(|| arguments.get_argument_at(0, "len").unwrap());

            let len = arg0.len() as u32;
            result.push_u32_shrunk(len);
            Ok(result)
        },
    )
}

pub(crate) fn create_pad_left_native_function() -> NativeFunction {
    NativeFunction::new(
        NativeFunctionSignature::new("pad_left"),
        |arguments, _| {
            let mut arg0 = arguments.get_argument_at(0, "pad_left")?.clone();
            let arg1 = arguments.get_argument_at(1, "pad_left")?;

            arg0.pad_left(arg1.as_usize_unsafe());

            Ok(arg0.clone())
        },
    )
}

pub(crate) fn create_pad_right_native_function() -> NativeFunction {
    NativeFunction::new(
        NativeFunctionSignature::new("pad_right"),
        |arguments, _| {
            let mut arg0: ByteBuffer = arguments.get_argument_at(0, "pad_right")?.clone();
            let arg1 = arguments.get_argument_at(1, "pad_right")?;

            arg0.pad_right(arg1.as_usize_unsafe());

            Ok(arg0.clone())
        },
    )
}

pub(crate) fn create_cmd_native_function() -> NativeFunction {
    NativeFunction::new(
        NativeFunctionSignature::new_unsafe("cmd"),
        |arguments, _| {
            let command = arguments.get_argument_at(0, "cmd")?
                .to_string()
                .map_err(|e| Error::Unknown(e.to_string()))?;

            let output = std::process::Command::new(command)
                .output()
                .map_err(|e| Error::Unknown(format!("Error executing command: {}", e)))?;

            let buffer = ByteBuffer::from(output.stdout);

            Ok(buffer)
        },
    )
}

pub(crate) fn create_read_file_native_function() -> NativeFunction {
    NativeFunction::new(
        NativeFunctionSignature::new("read_file"),
        |arguments, _| {
            let arg0 = arguments.get_argument_at(0, "read_file")?;

            let file_path = arg0
                .to_string()
                .map_err(|e| Error::Unknown(e.to_string()))?;

            let mut file = File::open(file_path)
                .map_err(|e| Error::Unknown(format!("Error executing command: {}", e)))?;

            let mut buf_string = String::new();
            file.read_to_string(&mut buf_string)
                .map_err(|e| Error::Unknown(format!("Error executing command: {}", e)))?;

            let buffer = ByteBuffer::from(buf_string.as_bytes().to_vec());

            Ok(buffer)
        },
    )
}

pub(crate) fn create_pad_native_function() -> NativeFunction {
    NativeFunction::new(
        NativeFunctionSignature::new("pad"),
        |arguments, _| {
            let mut buffer = arguments.get_argument_at(0, "pad")?.clone();

            let left_padding = arguments.get_named_argument("left").map(|b| b.as_usize_unsafe());
            let right_padding = arguments.get_named_argument("right").map(|b| b.as_usize_unsafe());

            if let Some(size) = left_padding {
                buffer.pad_left(size);
            }
            if let Some(size) = right_padding {
                buffer.pad_right(size);
            }

            Ok(buffer)
        },
    )
}

pub(crate) fn get_argument_at<'a>(
    arguments: &'a HashMap<String, ByteBuffer>,
    pos: usize,
    fn_name: &str,
) -> Result<&'a ByteBuffer, Error> {
    arguments
        .get(&pos.to_string())
        .ok_or_else(|| Error::MissingArgument {
            name: pos.to_string(),
            available_arguments: arguments.keys().cloned().collect(),
            function_name: fn_name.to_string(),
        })
}

pub(crate) fn get_named_argument<'a>(
    arguments: &'a HashMap<String, ByteBuffer>,
    name: &str,
) -> Option<&'a ByteBuffer> {
    arguments.get(name)
}
