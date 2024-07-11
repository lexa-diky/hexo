use std::collections::HashMap;
use crate::compiler::native_fn::Error;
use crate::util::byte_buffer::ByteBuffer;

pub(crate) struct NativeFunctionArguments<'a> {
    args: &'a HashMap<String, ByteBuffer>,
}

impl NativeFunctionArguments<'_> {
    pub(crate) fn new(args: &HashMap<String, ByteBuffer>) -> NativeFunctionArguments {
        NativeFunctionArguments {
            args
        }
    }

    pub(crate) fn get_argument_at(
        &self,
        pos: usize,
        fn_name: &str,
    ) -> Result<&ByteBuffer, Error> {
        let arguments = &self.args;
        arguments.get(&pos.to_string())
            .ok_or_else(move || Error::MissingArgument {
                name: pos.to_string(),
                available_arguments: arguments.keys().cloned().collect(),
                function_name: fn_name.to_string(),
            })
    }

    pub(crate) fn get_named_argument(
        &self,
        name: &str,
    ) -> Option<&ByteBuffer> {
        self.args.get(name)
    }
}