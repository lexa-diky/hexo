use crate::compiler::native_fn::{create_cmd_native_function, create_len_native_function, create_pad_left_native_function, create_pad_right_native_function};
use crate::compiler::native_fn::signature::NativeFunction;

#[derive(Clone, Debug)]
pub(crate) struct NativeFunctionIndex {
    functions: Vec<NativeFunction>,
}

impl NativeFunctionIndex {

    pub(crate) fn new() -> NativeFunctionIndex {
        return NativeFunctionIndex {
            functions: Self::create_native_functions(),
        };
    }

    pub(crate) fn find(&self, name: String) -> Option<&NativeFunction> {
        return self.functions.iter().find(|f| f.signature.name == name);
    }

    fn create_native_functions() -> Vec<NativeFunction> {
        return vec![
            create_len_native_function(),
            create_pad_left_native_function(),
            create_pad_right_native_function(),
            create_cmd_native_function()
        ];
    }
}
