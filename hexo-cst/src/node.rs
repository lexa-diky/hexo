use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct CstFile {
    pub(crate) path: PathBuf,
    pub(crate) main: CstFunctionStatement,
}

impl CstFile {
    pub(super) fn new(path: PathBuf, main: CstFunctionStatement) -> Self {
        CstFile { path, main }
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn main(&self) -> &CstFunctionStatement {
        &self.main
    }
}

#[derive(Clone, Debug)]
pub enum CstAtom {
    Hex(u8),
    String(String),
    Number(u32),
    Constant {
        name: String,
    },
    Function {
        name: String,
        params: Vec<CstActualParameter>,
    },
}

#[derive(Clone, Debug)]
pub struct CstActualParameter {
    name: String,
    value: CstAtomVec,
}

impl CstActualParameter {
    pub fn new(name: String, value: CstAtomVec) -> Self {
        CstActualParameter { name, value }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &CstAtomVec {
        &self.value
    }
}

pub type CstAtomVec = Vec<CstAtom>;

#[derive(Clone, Debug)]
pub struct CstEmitStatement {
    atoms: CstAtomVec,
}

impl CstEmitStatement {

    pub fn new(atoms: CstAtomVec) -> Self {
        CstEmitStatement { atoms }
    }

    pub fn atoms(&self) -> &CstAtomVec {
        &self.atoms
    }
}

#[derive(Clone, Debug)]
pub struct CstConstantStatement {
    name: String,
    atoms: CstAtomVec,
}

impl CstConstantStatement {

    pub fn new(name: String, atoms: CstAtomVec) -> Self {
        CstConstantStatement { name, atoms }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn atoms(&self) -> &CstAtomVec {
        &self.atoms
    }
}

#[derive(Clone, Debug)]
pub struct CstFunctionStatement {
    name: String,
    emits: Vec<CstEmitStatement>,
    functions: Vec<CstFunctionStatement>,
    constants: Vec<CstConstantStatement>,
}

impl CstFunctionStatement {

    pub fn new(
        name: String,
        emits: Vec<CstEmitStatement>,
        functions: Vec<CstFunctionStatement>,
        constants: Vec<CstConstantStatement>,
    ) -> Self {
        CstFunctionStatement {
            name,
            emits,
            functions,
            constants,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn emits(&self) -> &Vec<CstEmitStatement> {
        &self.emits
    }

    pub fn constants(&self) -> &Vec<CstConstantStatement> {
        &self.constants
    }

    pub fn functions(&self) -> &Vec<CstFunctionStatement> {
        &self.functions
    }
}