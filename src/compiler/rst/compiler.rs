use crate::util::byte_buffer::ByteBuffer;
use std::collections::HashMap;
use std::path::Path;
use crate::compiler::cst::{
    CstActualParameter, CstAtom, CstAtomVec, CstEmitStatement, CstFile, CstFunctionStatement,
};
use crate::compiler::rst::scope::{
    CompilationScope, ConstantBinding, FunctionBinding,
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
        let scope_id = HexoId::next();
        let mut scope = self.build_scope(scope_id, cst.path(), cst.main())?;

        let bb = self.build_bytes(scope_id, &mut scope, cst.main().emits())?;

        Ok(HexoFile::new(
            cst.path(),
            scope,
            bb,
        ))
    }

    fn build_bytes(
        &self,
        scope_id: HexoId,
        scope: &mut CompilationScope,
        emits: &Vec<CstEmitStatement>,
    ) -> Result<ByteBuffer, Error> {
        let mut byte_buffer = ByteBuffer::default();

        for emit in emits {
            self.build_bytes_into(scope_id, scope, emit.atoms(), &mut byte_buffer)?
        }

        Ok(byte_buffer)
    }

    fn build_bytes_into(
        &self,
        scope_id: HexoId,
        scope: &mut CompilationScope,
        atoms: &CstAtomVec,
        buffer: &mut ByteBuffer,
    ) -> Result<(), Error> {
        for atom in atoms {
            match atom {
                CstAtom::Hex(byte) => buffer.push_byte(*byte),
                CstAtom::String(string) => buffer.push_string(string.clone()),
                CstAtom::Number(number) => buffer.push_u32_shrunk(*number),
                CstAtom::Constant { name } => {
                    Self::build_constant_into(scope_id, scope, name, buffer)?
                }
                CstAtom::Function { name, params } => {
                    self.build_function_into(scope_id, scope, name.clone(), params, buffer)?
                }
            }
        }

        Ok(())
    }

    fn build_function_into(
        &self,
        scope_id: HexoId,
        scope: &mut CompilationScope,
        function_name: String,
        params: &Vec<CstActualParameter>,
        buffer: &mut ByteBuffer,
    ) -> Result<(), Error> {
        let native_function = scope.get_native_function(function_name.as_str());
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
                self.build_bytes_into(scope_id, scope, param.value(), &mut param_buffer)?;

                params_buffer.insert(param.name().to_string(), param_buffer);
            }

            executor(&params_buffer, self.parent)
                .map(|bb| buffer.push_byte_buffer(&bb))
                .map_err(Error::NativeFunctionExecution)?;

            return Ok(());
        }

        let binding = scope.clone();
        let function_binding = binding
            .get_local_function(scope_id, &function_name)
            .ok_or(Error::UnresolvedFunction {
                name: function_name.clone(),
            })?;

        for param in params {
            let mut param_buffer = ByteBuffer::default();
            self.build_bytes_into(scope_id, scope, param.value(), &mut param_buffer).unwrap();

            scope.bind_local_constant(
                function_binding.identifier,
                ConstantBinding {
                    name: param.name().to_string(),
                    byte_buffer: param_buffer,
                },
            );
        }

        for emit in &function_binding.emits {
            self.build_bytes_into(function_binding.identifier, scope, emit.atoms(), buffer)
                .unwrap();
        }

        Ok(())
    }

    fn build_constant_into(
        scope_id: HexoId,
        scope: &CompilationScope,
        name: &String,
        buffer: &mut ByteBuffer,
    ) -> Result<(), Error> {
        let constant_binding = scope
            .get_local_constant(scope_id, name)
            .ok_or(Error::UnresolvedConstant { name: name.clone() })?;

        buffer.push_byte_buffer(&constant_binding.byte_buffer);

        Ok(())
    }

    fn build_scope(
        &self,
        scope_id: HexoId,
        file_path: &Path,
        cst: &CstFunctionStatement,
    ) -> Result<CompilationScope, Error> {
        let mut root_scope = CompilationScope::new(file_path);

        self.build_scope_into(scope_id, &cst, &mut root_scope)?;

        Ok(root_scope)
    }

    fn build_scope_into(
        &self,
        scope_id: HexoId,
        cst: &&CstFunctionStatement,
        root_scope: &mut CompilationScope,
    ) -> Result<(), Error> {
        self.build_scope_constants_into(scope_id, cst, root_scope)?;
        self.build_scope_functions_into(scope_id, cst, root_scope)?;
        Ok(())
    }

    fn build_scope_constants_into(
        &self,
        scope_id: HexoId,
        cst: &&CstFunctionStatement,
        scope: &mut CompilationScope,
    ) -> Result<(), Error> {
        for constant in cst.constants() {
            let mut buff = ByteBuffer::default();
            self.build_bytes_into(scope_id, scope, constant.atoms(), &mut buff)?;
            scope.bind_local_constant(
                scope_id,
                ConstantBinding {
                    name: constant.name().to_string(),
                    byte_buffer: buff,
                },
            )
        }

        Ok(())
    }

    fn build_scope_functions_into(
        &self,
        scope_id: HexoId,
        cst: &&CstFunctionStatement,
        root_scope: &mut CompilationScope,
    ) -> Result<(), Error> {
        for function in cst.functions() {
            let inner_function_scope_id = HexoId::next();
            root_scope.bind_local_function(
                scope_id,
                FunctionBinding {
                    identifier: inner_function_scope_id,
                    name: function.name().to_string(),
                    emits: function.emits().clone(),
                },
            );

            self.build_scope_into(inner_function_scope_id, &function, root_scope)?;

            root_scope.bind_parents(inner_function_scope_id, vec![scope_id]);
        }

        Ok(())
    }
}
