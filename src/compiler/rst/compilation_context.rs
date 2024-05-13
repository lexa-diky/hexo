use crate::compiler::cst::CstFile;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::compiler::util::ByteBuffer;

pub(crate) struct ConstantBinding {
    pub(crate) name: String,
    pub(crate) byte_buffer: ByteBuffer,
}

pub(crate) struct CompilationContext {
    self_path: PathBuf,
    constant_table: HashMap<String, ConstantBinding>,
}

impl CompilationContext {
    pub(crate) fn new(path: &PathBuf) -> CompilationContext {
        return CompilationContext {
            self_path: path.clone(),
            constant_table: HashMap::new(),
        };
    }

    pub(crate) fn bind_constant(&mut self, constant: ConstantBinding) {
        self.constant_table.insert(constant.name.clone(), constant);
    }

    pub(crate) fn has_constant(&self, name: &String) -> bool {
        return self.constant_table.contains_key(name);
    }

    pub(crate) fn get_constant(&self, name: &String) -> Option<&ConstantBinding> {
        return self.constant_table.get(name);
    }
}
