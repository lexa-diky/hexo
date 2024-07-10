use crate::util::byte_buffer::ByteBuffer;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use crate::compiler::compiler_source::LiteralCompilerSource;
use crate::compiler::native_fn::error::Error;
use crate::compiler::native_fn::signature::{NativeFunction, NativeFunctionSignature};

pub(crate) fn create_len_native_function() -> NativeFunction {
    NativeFunction::new(
        NativeFunctionSignature::new("len"),
        |arguments, _| {
            let mut result = ByteBuffer::default();
            let arg0 = get_named_argument(&arguments, "utf8")
                .unwrap_or_else(|| get_argument_at(&arguments, 0, "len").unwrap());

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
            let mut arg0 = get_argument_at(&arguments, 0, "pad_left")?.clone();
            let arg1 = get_argument_at(&arguments, 1, "pad_left")?;

            arg0.pad_left(arg1.as_usize_unsafe());

            Ok(arg0.clone())
        },
    )
}

pub(crate) fn create_pad_right_native_function() -> NativeFunction {
    NativeFunction::new(
        NativeFunctionSignature::new("pad_right"),
        |arguments, _| {
            let mut arg0: ByteBuffer = get_argument_at(&arguments, 0, "pad_right")?.clone();
            let arg1 = get_argument_at(&arguments, 1, "pad_right")?;

            arg0.pad_right(arg1.as_usize_unsafe());

            Ok(arg0.clone())
        },
    )
}

pub(crate) fn create_cmd_native_function() -> NativeFunction {
    NativeFunction::new(
        NativeFunctionSignature::new_unsafe("cmd"),
        |arguments, _| {
            let command = get_argument_at(&arguments, 0, "cmd")?
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
            let arg0 = get_argument_at(&arguments, 0, "read_file")?;

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
            let mut buffer = get_argument_at(&arguments, 0, "pad")?.clone();

            let left_padding = get_named_argument(&arguments, "left").map(|b| b.as_usize_unsafe());
            let right_padding =
                get_named_argument(&arguments, "right").map(|b| b.as_usize_unsafe());

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

pub(crate) fn create_eval_native_function() -> NativeFunction {
    NativeFunction::new(
        NativeFunctionSignature::new_unsafe("eval"),
        |arguments, compiler| {
            let buffer = get_argument_at(&arguments, 0, "eval")?.clone();
            let source = LiteralCompilerSource::anonymous(
                buffer.to_string()
                    .map_err(|e| Error::Unknown(e.to_string()))?,
            );
            let result = compiler.compile(&source)
                .map_err(|e| Error::Unknown(e.to_string()))?;
            Ok(ByteBuffer::from(result.content))
        },
    )
}

fn get_argument_at<'a>(
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

fn get_named_argument<'a>(
    arguments: &'a HashMap<String, ByteBuffer>,
    name: &str,
) -> Option<&'a ByteBuffer> {
    arguments.get(name)
}
