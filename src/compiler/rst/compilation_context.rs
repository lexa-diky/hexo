use crate::compiler::cst::{CstConstantStatement, CstEmitStatement, CstFile, CstFunctionStatement};
use std::collections::HashMap;
use std::path::PathBuf;
use pest::pratt_parser::Op;

use crate::compiler::util::ByteBuffer;

#[derive(Clone, Debug)]
pub(crate) struct ConstantBinding {
    pub(crate) name: String,
    pub(crate) byte_buffer: ByteBuffer,
}

#[derive(Clone, Debug)]
pub(crate) struct FunctionBinding {
    pub(crate) identifier: u64,
    pub(crate) name: String,
    pub(crate) emits: Vec<CstEmitStatement>,
}

#[derive(Clone, Debug)]
pub(crate) struct LocalCompilationContext {
    constant_table: HashMap<String, ConstantBinding>,
    function_table: HashMap<String, FunctionBinding>,
    parents: Vec<u64>,
}

#[derive(Clone, Debug)]
pub(crate) struct CompilationContext {
    self_path: PathBuf,
    local_contexts: HashMap<u64, LocalCompilationContext>,
}

impl CompilationContext {
    pub(crate) fn new(path: &PathBuf) -> CompilationContext {
        return CompilationContext {
            self_path: path.clone(),
            local_contexts: HashMap::new(),
        };
    }

    // region constant
    pub(crate) fn bind_local_constant(&mut self, context_id: u64, constant: ConstantBinding) {
        if !self.local_contexts.contains_key(&context_id) {
            self.local_contexts.insert(context_id, LocalCompilationContext::new());
        }

        let mut local_context: &mut LocalCompilationContext = self.local_contexts.get_mut(&context_id)
            .expect("prechecked but value is still missing");

        local_context.bind_constant(constant);
    }

    pub(crate) fn get_local_constant(&self, context_id: u64, name: &String) -> Option<&ConstantBinding> {
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

        return None;
    }

    // endregion

    pub(crate) fn bind_local_function(&mut self, context_id: u64, function: FunctionBinding) {
        if !self.local_contexts.contains_key(&context_id) {
            self.local_contexts.insert(context_id, LocalCompilationContext::new());
        }

        let mut local_context: &mut LocalCompilationContext = self.local_contexts.get_mut(&context_id)
            .expect("prechecked but value is still missing");

        local_context.bind_function(function);
    }

    pub(crate) fn get_local_function(&self, context_id: u64, name: &String) -> Option<&FunctionBinding> {
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

        return None;
    }

    pub(crate) fn get_parents(&self, context_id: u64) -> Option<Vec<u64>> {
        return self.local_contexts.get(&context_id)
            .map(|it| it.parents.clone());
    }

    pub(crate) fn bind_parents(&mut self, context_id: u64, parents: Vec<u64>) {
        if !self.local_contexts.contains_key(&context_id) {
            self.local_contexts.insert(context_id, LocalCompilationContext::new());
        }

        let mut local_context: &mut LocalCompilationContext = self.local_contexts.get_mut(&context_id)
            .expect("prechecked but value is still missing");

        for parent in parents {
            local_context.attach_parent(parent);
        }
    }
}

impl LocalCompilationContext {
    fn new() -> LocalCompilationContext {
        return LocalCompilationContext {
            constant_table: HashMap::new(),
            function_table: HashMap::new(),
            parents: Vec::new(),
        };
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

    fn attach_parent(&mut self, parent_id: u64) {
        self.parents.push(parent_id);
    }
}