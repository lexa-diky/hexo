use crate::compiler::native_fn::signature::NativeFunction;
use crate::compiler::native_fn::{create_cmd_native_function, create_eval_native_function, create_len_native_function, create_pad_left_native_function, create_pad_native_function, create_pad_right_native_function, create_read_file_native_function};

#[derive(Clone, Debug)]
pub(crate) struct NativeFunctionIndex {
    functions: Vec<NativeFunction>,
}

impl NativeFunctionIndex {

    pub(crate) fn new() -> NativeFunctionIndex {
        NativeFunctionIndex {
            functions: Self::create_native_functions(),
        }
    }

    pub(crate) fn find(&self, name: String) -> Option<&NativeFunction> {
        return self.functions.iter().find(|f| f.signature().name() == name);
    }

    fn create_native_functions() -> Vec<NativeFunction> {
        vec![
            create_len_native_function(),
            create_pad_left_native_function(),
            create_pad_right_native_function(),
            create_cmd_native_function(),
            create_read_file_native_function(),
            create_pad_native_function(),
            create_eval_native_function()
        ]
    }
}
