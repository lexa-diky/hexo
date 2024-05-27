use crate::compiler::cst::{CstConstantStatement, CstEmitStatement, CstFile, CstFunctionStatement};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::compiler::util::ByteBuffer;

#[derive(Clone)]
pub(crate) struct ConstantBinding {
    pub(crate) name: String,
    pub(crate) byte_buffer: ByteBuffer,
}

#[derive(Clone)]
pub(crate) struct FunctionBinding {
    pub(crate) name: String,
    pub(crate) context: CompilationContext
}

#[derive(Clone)]
pub(crate) struct CompilationContext {
    self_path: PathBuf,
    parent: Option<Box<CompilationContext>>,
    constant_table: HashMap<String, ConstantBinding>,
    function_table: HashMap<String, FunctionBinding>,
}

impl CompilationContext {
    pub(crate) fn new(path: &PathBuf) -> CompilationContext {
        return CompilationContext {
            self_path: path.clone(),
            parent: None,
            constant_table: HashMap::new(),
            function_table: HashMap::new(),
        };
    }

    pub(crate) fn branch(parent: CompilationContext) -> CompilationContext {
        return CompilationContext {
            self_path: parent.self_path.clone(),
            parent: Some(Box::new(parent)),
            constant_table: HashMap::new(),
            function_table: HashMap::new(),
        };
    }

    pub(crate) fn bind_constant(&mut self, constant: ConstantBinding) {
        self.constant_table.insert(constant.name.clone(), constant);
    }

    pub(crate) fn get_constant(&self, name: &String) -> Option<&ConstantBinding> {
        return self.constant_table.get(name);
    }

    pub(crate) fn bind_function(&mut self, function: FunctionBinding) {
        self.function_table.insert(function.name.clone(), function);
    }

    pub(crate) fn get_function(&self, name: &String) -> Option<&FunctionBinding> {
        return self.function_table.get(name);
    }
}
