use crate::compiler::cst::CstEmitStatement;
use crate::compiler::native_fn::{NativeFunction, NativeFunctionIndex};
use hexo_id::HexoId;
use hexo_io::byte_buffer::ByteBuffer;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub(crate) struct ConstantBinding {
    pub(crate) name: String,
    pub(crate) byte_buffer: ByteBuffer,
}

#[derive(Clone, Debug)]
pub(crate) struct FunctionBinding {
    pub(crate) identifier: HexoId,
    pub(crate) name: String,
    pub(crate) emits: Vec<CstEmitStatement>,
}

#[derive(Clone, Debug)]
pub(crate) struct LocalCompilationContext {
    constant_table: HashMap<String, ConstantBinding>,
    function_table: HashMap<String, FunctionBinding>,
    parents: Vec<HexoId>,
}

#[derive(Clone, Debug)]
pub(crate) struct CompilationContext {
    self_path: PathBuf,
    local_contexts: HashMap<HexoId, LocalCompilationContext>,
    native_function_index: NativeFunctionIndex,
}

impl CompilationContext {
    pub(crate) fn new(path: &Path) -> CompilationContext {
        CompilationContext {
            self_path: path.to_path_buf(),
            local_contexts: HashMap::new(),
            native_function_index: NativeFunctionIndex::new(),
        }
    }

    // region constant
    pub(crate) fn bind_local_constant(&mut self, context_id: HexoId, constant: ConstantBinding) {
        self.local_contexts
            .entry(context_id)
            .or_insert_with(LocalCompilationContext::new);

        let local_context: &mut LocalCompilationContext = self
            .local_contexts
            .get_mut(&context_id)
            .expect("prechecked but value is still missing");

        local_context.bind_constant(constant);
    }

    pub(crate) fn get_local_constant(
        &self,
        context_id: HexoId,
        name: &String,
    ) -> Option<&ConstantBinding> {
        let local_context = self.local_contexts.get(&context_id)?;

        let local_constant = local_context.get_constant(name);

        if local_constant.is_none() {
            for parent in &local_context.parents {
                let parent_constant = self.get_local_constant(*parent, name);
                if parent_constant.is_some() {
                    return parent_constant;
                }
            }
        } else {
            return local_constant;
        }

        None
    }

    // endregion

    pub(crate) fn bind_local_function(&mut self, context_id: HexoId, function: FunctionBinding) {
        self.local_contexts
            .entry(context_id)
            .or_insert_with(LocalCompilationContext::new);

        let local_context: &mut LocalCompilationContext = self
            .local_contexts
            .get_mut(&context_id)
            .expect("prechecked but value is still missing");

        local_context.bind_function(function);
    }

    pub(crate) fn get_local_function(
        &self,
        context_id: HexoId,
        name: &String,
    ) -> Option<&FunctionBinding> {
        let local_context = self.local_contexts.get(&context_id)?;

        let local_function = local_context.get_function(name);

        if local_function.is_none() {
            for parent in &local_context.parents {
                let parent_function = self.get_local_function(*parent, name);
                if parent_function.is_some() {
                    return parent_function;
                }
            }
        } else {
            return local_function;
        }

        None
    }

    pub(crate) fn get_native_function(&self, name: String) -> Option<&NativeFunction> {
        return self.native_function_index.find(name);
    }

    pub(crate) fn bind_parents(&mut self, context_id: HexoId, parents: Vec<HexoId>) {
        self.local_contexts
            .entry(context_id)
            .or_insert_with(LocalCompilationContext::new);

        let local_context: &mut LocalCompilationContext = self
            .local_contexts
            .get_mut(&context_id)
            .expect("prechecked but value is still missing");

        for parent in parents {
            local_context.attach_parent(parent);
        }
    }
}

impl LocalCompilationContext {
    fn new() -> LocalCompilationContext {
        LocalCompilationContext {
            constant_table: HashMap::new(),
            function_table: HashMap::new(),
            parents: Vec::new(),
        }
    }

    fn bind_constant(&mut self, constant: ConstantBinding) {
        self.constant_table.insert(constant.name.clone(), constant);
    }

    fn get_constant(&self, name: &String) -> Option<&ConstantBinding> {
        return self.constant_table.get(name);
    }

    fn bind_function(&mut self, function: FunctionBinding) {
        self.function_table.insert(function.name.clone(), function);
    }

    fn get_function(&self, name: &String) -> Option<&FunctionBinding> {
        return self.function_table.get(name);
    }

    fn attach_parent(&mut self, parent_id: HexoId) {
        self.parents.push(parent_id);
    }
}
