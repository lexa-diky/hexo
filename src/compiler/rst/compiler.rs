use crate::util::byte_buffer::ByteBuffer;
use std::collections::HashMap;
use std::path::Path;
use crate::compiler::cst::{
    CstActualParameter, CstAtom, CstAtomVec, CstEmitStatement, CstFile, CstFunctionStatement,
};
use crate::compiler::rst::compilation_context::{
    CompilationContext, ConstantBinding, FunctionBinding,
};
use crate::compiler::rst::error::Error;
use crate::compiler::rst::node::HexoFile;
use crate::compiler::HexoCompiler;
use crate::util::id::HexoId;

pub(crate) struct RstCompiler<'a> {
    parent: &'a HexoCompiler,
    safe_mode: bool,
}

impl RstCompiler<'_> {
    pub(crate) fn new(parent: &HexoCompiler, safe_mode: bool) -> RstCompiler {
        RstCompiler { parent, safe_mode }
    }

    pub(crate) fn compile(&self, cst: &CstFile) -> Result<HexoFile, Error> {
        let context_id = HexoId::next();
        let mut context = self.build_context(context_id, cst.path(), cst.main())?;

        let bb = self.build_bytes(context_id, &mut context, cst.main().emits())?;

        Ok(HexoFile::new(
            cst.path(),
            context,
            bb,
        ))
    }

    fn build_bytes(
        &self,
        context_id: HexoId,
        context: &mut CompilationContext,
        emits: &Vec<CstEmitStatement>,
    ) -> Result<ByteBuffer, Error> {
        let mut byte_buffer = ByteBuffer::default();

        for emit in emits {
            self.build_bytes_into(context_id, context, emit.atoms(), &mut byte_buffer)?
        }

        Ok(byte_buffer)
    }

    fn build_bytes_into(
        &self,
        context_id: HexoId,
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
                    self.build_function_into(context_id, context, name.clone(), params, buffer)?
                }
            }
        }

        Ok(())
    }

    fn build_function_into(
        &self,
        context_id: HexoId,
        context: &mut CompilationContext,
        function_name: String,
        params: &Vec<CstActualParameter>,
        buffer: &mut ByteBuffer,
    ) -> Result<(), Error> {
        let native_function = context.get_native_function(function_name.as_str());
        if let Some(native_function) = native_function {
            if self.safe_mode && !native_function.signature().is_safe() {
                return Err(
                    Error::NativeFunctionIsUnsafe {
                        name: native_function.signature().name().to_string()
                    }
                )
            }

            let executor = native_function.executor();
            let mut params_buffer = HashMap::new();

            for param in params {
                let mut param_buffer = ByteBuffer::default();
                self.build_bytes_into(context_id, context, param.value(), &mut param_buffer)?;

                params_buffer.insert(param.name().to_string(), param_buffer);
            }

            executor(&params_buffer, self.parent)
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
            let mut param_buffer = ByteBuffer::default();
            self.build_bytes_into(context_id, context, param.value(), &mut param_buffer).unwrap();

            context.bind_local_constant(
                function_binding.identifier,
                ConstantBinding {
                    name: param.name().to_string(),
                    byte_buffer: param_buffer,
                },
            );
        }

        for emit in &function_binding.emits {
            self.build_bytes_into(function_binding.identifier, context, emit.atoms(), buffer)
                .unwrap();
        }

        Ok(())
    }

    fn build_constant_into(
        context_id: HexoId,
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
        &self,
        context_id: HexoId,
        file_path: &Path,
        cst: &CstFunctionStatement,
    ) -> Result<CompilationContext, Error> {
        let mut root_context = CompilationContext::new(file_path);

        self.build_context_into(context_id, &cst, &mut root_context)?;

        Ok(root_context)
    }

    fn build_context_into(
        &self,
        context_id: HexoId,
        cst: &&CstFunctionStatement,
        root_context: &mut CompilationContext,
    ) -> Result<(), Error> {
        self.build_context_constants_into(context_id, cst, root_context)?;
        self.build_context_functions_into(context_id, cst, root_context)?;
        Ok(())
    }

    fn build_context_constants_into(
        &self,
        context_id: HexoId,
        cst: &&CstFunctionStatement,
        context: &mut CompilationContext,
    ) -> Result<(), Error> {
        for constant in cst.constants() {
            let mut buff = ByteBuffer::default();
            self.build_bytes_into(context_id, context, constant.atoms(), &mut buff)?;
            context.bind_local_constant(
                context_id,
                ConstantBinding {
                    name: constant.name().to_string(),
                    byte_buffer: buff,
                },
            )
        }

        Ok(())
    }

    fn build_context_functions_into(
        &self,
        context_id: HexoId,
        cst: &&CstFunctionStatement,
        root_context: &mut CompilationContext,
    ) -> Result<(), Error> {
        for function in cst.functions() {
            let inner_function_context_id = HexoId::next();
            root_context.bind_local_function(
                context_id,
                FunctionBinding {
                    identifier: inner_function_context_id,
                    name: function.name().to_string(),
                    emits: function.emits().clone(),
                },
            );

            self.build_context_into(inner_function_context_id, &function, root_context)?;

            root_context.bind_parents(inner_function_context_id, vec![context_id]);
        }

        Ok(())
    }
}
