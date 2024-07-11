use crate::compiler::compiler_source::LiteralCompilerSource;
use crate::compiler::native_fn::{
    Error, NativeFunction, NativeFunctionDefinition, NativeFunctionSignature,
};
use crate::util::byte_buffer::ByteBuffer;

#[derive(Default)]
pub(crate) struct EvalNativeFunctionDef {}

impl NativeFunctionDefinition for EvalNativeFunctionDef {
    fn create(&self) -> NativeFunction {
        NativeFunction::new(
            NativeFunctionSignature::new_unsafe("eval"),
            |arguments, compiler| {
                let buffer = arguments.get_argument_at(0, "eval")?.clone();
                let source = LiteralCompilerSource::anonymous(
                    buffer
                        .to_string()
                        .map_err(|e| Error::Unknown(e.to_string()))?,
                );
                let result = compiler
                    .compile(&source)
                    .map_err(|e| Error::Unknown(e.to_string()))?;
                Ok(ByteBuffer::from(result.content))
            },
        )
    }
}
