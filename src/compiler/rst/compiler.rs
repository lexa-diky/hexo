use std::collections::HashMap;
use std::path::PathBuf;

use crate::compiler::cst::{
    CstActualParameter, CstAtom, CstAtomVec, CstEmitStatement, CstFile, CstFunctionStatement,
};
use crate::compiler::rst::compilation_context::{
    CompilationContext, ConstantBinding, FunctionBinding,
};
use crate::compiler::rst::node::HexoFile;
use crate::compiler::util::{ByteBuffer, next_identifier};
use crate::compiler::HexoCompiler;
use crate::compiler::rst::error::Error;

pub(crate) struct RstCompiler<'a> {
    parent: &'a HexoCompiler,
}

impl RstCompiler<'_> {
    pub(crate) fn new(parent: &HexoCompiler) -> RstCompiler {
        RstCompiler { parent }
    }

    pub(crate) fn compile(&self, cst: &CstFile) -> Result<HexoFile, Error> {
        let context_id = next_identifier();
        let mut context = Self::build_context(context_id, &cst.path, &cst.main)?;

        let bb = Self::build_bytes(context_id, &mut context, &cst.main.emits)?;

        Ok(HexoFile {
            path: cst.path.clone(),
            context,
            emits: bb,
        })
    }

    fn build_bytes(
        context_id: u64,
        context: &mut CompilationContext,
        emits: &Vec<CstEmitStatement>,
    ) -> Result<ByteBuffer, Error> {
        let mut byte_buffer = ByteBuffer::new();

        for emit in emits {
            Self::build_bytes_into(context_id, context, &emit.atoms, &mut byte_buffer)?
        }

        Ok(byte_buffer)
    }

    fn build_bytes_into(
        context_id: u64,
        context: &mut CompilationContext,
        atoms: &CstAtomVec,
        buffer: &mut ByteBuffer,
    ) -> Result<(), Error> {
        for atom in atoms {
            match atom {
                CstAtom::Hex(byte) => buffer.push_byte(*byte),
                CstAtom::String(string) => buffer.push_string(string.clone()),
                CstAtom::Number(number) => buffer.push_u32_shrunk(*number),
                CstAtom::Constant { name } => {
                    Self::build_constant_into(context_id, context, name, buffer)?
                }
                CstAtom::Function { name, params } => {
                    Self::build_function_into(context_id, context, name.clone(), params, buffer)?
                }
            }
        }

        Ok(())
    }

    fn build_function_into(
        context_id: u64,
        context: &mut CompilationContext,
        function_name: String,
        params: &Vec<CstActualParameter>,
        buffer: &mut ByteBuffer,
    ) -> Result<(), Error> {
        let native_function = context.get_native_function(function_name.clone());
        if native_function.is_some() {
            let executor = native_function.unwrap().executor;
            let mut params_buffer = HashMap::new();

            for param in params {
                let mut param_buffer = ByteBuffer::new();
                Self::build_bytes_into(context_id, context, &param.value, &mut param_buffer)
                    .unwrap();

                params_buffer.insert(param.name.clone(), param_buffer);
            }

            executor(params_buffer.clone())
                .map(|bb| buffer.push_byte_buffer(&bb))
                .map_err(Error::NativeFunctionExecution)?;

            return Ok(());
        }

        let binding = context.clone();
        let function_binding = binding
            .get_local_function(context_id, &function_name)
            .ok_or(Error::UnresolvedFunction {
                name: function_name.clone(),
            })?;

        for param in params {
            let mut param_buffer = ByteBuffer::new();
            Self::build_bytes_into(context_id, context, &param.value, &mut param_buffer).unwrap();

            context.bind_local_constant(
                function_binding.identifier,
                ConstantBinding {
                    name: param.name.clone(),
                    byte_buffer: param_buffer,
                },
            );
        }

        for emit in &function_binding.emits {
            Self::build_bytes_into(function_binding.identifier, context, &emit.atoms, buffer)
                .unwrap();
        }

        Ok(())
    }

    fn build_constant_into(
        context_id: u64,
        context: &CompilationContext,
        name: &String,
        buffer: &mut ByteBuffer,
    ) -> Result<(), Error> {
        let constant_binding = context
            .get_local_constant(context_id, name)
            .ok_or(Error::UnresolvedConstant { name: name.clone() })?;

        buffer.push_byte_buffer(&constant_binding.byte_buffer);

        Ok(())
    }

    fn build_context(
        context_id: u64,
        file_path: &PathBuf,
        cst: &CstFunctionStatement,
    ) -> Result<CompilationContext, Error> {
        let mut root_context = CompilationContext::new(file_path);

        Self::build_context_into(context_id, &cst, &mut root_context)?;

        Ok(root_context)
    }

    fn build_context_into(
        context_id: u64,
        cst: &&CstFunctionStatement,
        root_context: &mut CompilationContext,
    ) -> Result<(), Error> {
        Self::build_context_constants_into(context_id, cst, root_context)?;
        Self::build_context_functions_into(context_id, cst, root_context)?;
        Ok(())
    }

    fn build_context_constants_into(
        context_id: u64,
        cst: &&CstFunctionStatement,
        context: &mut CompilationContext,
    ) -> Result<(), Error> {
        for constant in &cst.constants {
            let mut buff = ByteBuffer::new();
            Self::build_bytes_into(context_id, context, &constant.atoms, &mut buff)?;
            context.bind_local_constant(
                context_id,
                ConstantBinding {
                    name: constant.name.clone(),
                    byte_buffer: buff,
                },
            )
        }

        Ok(())
    }

    fn build_context_functions_into(
        context_id: u64,
        cst: &&CstFunctionStatement,
        root_context: &mut CompilationContext,
    ) -> Result<(), Error> {
        for function in &cst.functions {
            let inner_function_context_id = next_identifier();
            root_context.bind_local_function(
                context_id,
                FunctionBinding {
                    identifier: inner_function_context_id,
                    name: function.name.clone(),
                    emits: function.emits.clone(),
                },
            );

            Self::build_context_into(inner_function_context_id, &function, root_context)?;

            root_context.bind_parents(inner_function_context_id, vec![context_id]);
        }

        Ok(())
    }
}
