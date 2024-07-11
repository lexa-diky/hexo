use crate::compiler::native_fn::signature::NativeFunction;
use crate::compiler::native_fn::{create_cmd_native_function, create_len_native_function, create_pad_left_native_function, create_pad_native_function, create_pad_right_native_function, create_read_file_native_function, NativeFunctionDefinition};
use crate::compiler::native_fn::implementation::EvalNativeFunctionDef;
#[derive(Clone, Debug)]
pub(crate) struct NativeFunctionIndex {
    functions: Vec<NativeFunction>,
}

impl Default for NativeFunctionIndex {
    fn default() -> Self {
        NativeFunctionIndex {
            functions: Self::create_native_functions(),
        }
    }
}

impl NativeFunctionIndex {

    pub(crate) fn find(&self, name: String) -> Option<&NativeFunction> {
        return self.functions.iter().find(|f| f.signature().name() == name);
    }

    fn create_native_functions() -> Vec<NativeFunction> {
        let definitions = Self::create_native_function_definitions();

        let mut direct_fn = vec![
            create_len_native_function(),
            create_pad_left_native_function(),
            create_pad_right_native_function(),
            create_cmd_native_function(),
            create_read_file_native_function(),
            create_pad_native_function(),
        ];
        direct_fn.append(
            &mut definitions.iter().map(|d| d.create()).collect()
        );

        direct_fn
    }

    fn create_native_function_definitions() -> Vec<Box<dyn NativeFunctionDefinition>> {
        return vec![
            Box::new(EvalNativeFunctionDef::default()),
        ]
    }
}
