use crate::compiler::cst::CstEmitStatement;
use crate::compiler::native_fn::{NativeFunction, NativeFunctionIndex};
use crate::util::id::HexoId;
use crate::util::byte_buffer::ByteBuffer;
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
pub(crate) struct LocalCompilationScope {
    constant_table: HashMap<String, ConstantBinding>,
    function_table: HashMap<String, FunctionBinding>,
    parents: Vec<HexoId>,
}

#[derive(Clone, Debug)]
pub(crate) struct CompilationScope {
    self_path: PathBuf,
    local_scopes: HashMap<HexoId, LocalCompilationScope>,
    native_function_index: NativeFunctionIndex,
}

impl CompilationScope {
    pub(crate) fn new(path: &Path) -> CompilationScope {
        CompilationScope {
            self_path: path.to_path_buf(),
            local_scopes: HashMap::new(),
            native_function_index: NativeFunctionIndex::new(),
        }
    }

    // region constant
    pub(crate) fn bind_local_constant(&mut self, scope_id: HexoId, constant: ConstantBinding) {
        self.local_scopes
            .entry(scope_id)
            .or_insert_with(LocalCompilationScope::new);

        let local_scope: &mut LocalCompilationScope = self
            .local_scopes
            .get_mut(&scope_id)
            .expect("prechecked but value is still missing");

        local_scope.bind_constant(constant);
    }

    pub(crate) fn get_local_constant(
        &self,
        scope_id: HexoId,
        name: &String,
    ) -> Option<&ConstantBinding> {
        let local_scope = self.local_scopes.get(&scope_id)?;

        let local_constant = local_scope.get_constant(name);

        if local_constant.is_none() {
            for parent in &local_scope.parents {
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

    pub(crate) fn bind_local_function(&mut self, scope_id: HexoId, function: FunctionBinding) {
        self.local_scopes
            .entry(scope_id)
            .or_insert_with(LocalCompilationScope::new);

        let local_scope: &mut LocalCompilationScope = self
            .local_scopes
            .get_mut(&scope_id)
            .expect("prechecked but value is still missing");

        local_scope.bind_function(function);
    }

    pub(crate) fn get_local_function(
        &self,
        scope_id: HexoId,
        name: &String,
    ) -> Option<&FunctionBinding> {
        let local_scope = self.local_scopes.get(&scope_id)?;

        let local_function = local_scope.get_function(name);

        if local_function.is_none() {
            for parent in &local_scope.parents {
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

    pub(crate) fn get_native_function(&self, name: &str) -> Option<&NativeFunction> {
        return self.native_function_index.find(name.to_string());
    }

    pub(crate) fn bind_parents(&mut self, scope_id: HexoId, parents: Vec<HexoId>) {
        self.local_scopes
            .entry(scope_id)
            .or_insert_with(LocalCompilationScope::new);

        let local_scope: &mut LocalCompilationScope = self
            .local_scopes
            .get_mut(&scope_id)
            .expect("prechecked but value is still missing");

        for parent in parents {
            local_scope.attach_parent(parent);
        }
    }
}

impl LocalCompilationScope {
    fn new() -> LocalCompilationScope {
        LocalCompilationScope {
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
