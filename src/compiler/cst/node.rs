use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub(crate) struct CstFile {
    pub(crate) path: PathBuf,
    pub(crate) main: CstFunctionStatement,
}

impl CstFile {
    pub(super) fn new(path: &Path, main: CstFunctionStatement) -> Self {
        CstFile { path: path.to_path_buf(), main: main }
    }

    pub(crate) fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub(crate) fn main(&self) -> &CstFunctionStatement {
        &self.main
    }
}

#[derive(Clone, Debug)]
pub(crate) enum CstAtom {
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
pub(crate) struct CstActualParameter {
    name: String,
    value: CstAtomVec,
}

impl CstActualParameter {
    pub(crate) fn new(name: String, value: CstAtomVec) -> Self {
        CstActualParameter { name, value }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn value(&self) -> &CstAtomVec {
        &self.value
    }
}

pub(crate) type CstAtomVec = Vec<CstAtom>;

#[derive(Clone, Debug)]
pub(crate) struct CstEmitStatement {
    atoms: CstAtomVec,
}

impl CstEmitStatement {

    pub(crate) fn new(atoms: CstAtomVec) -> Self {
        CstEmitStatement { atoms }
    }

    pub(crate) fn atoms(&self) -> &CstAtomVec {
        &self.atoms
    }
}

#[derive(Clone, Debug)]
pub(crate) struct CstConstantStatement {
    name: String,
    atoms: CstAtomVec,
}

impl CstConstantStatement {

    pub(crate) fn new(name: String, atoms: CstAtomVec) -> Self {
        CstConstantStatement { name, atoms }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn atoms(&self) -> &CstAtomVec {
        &self.atoms
    }
}

#[derive(Clone, Debug)]
pub(crate) struct CstFunctionStatement {
    name: String,
    emits: Vec<CstEmitStatement>,
    functions: Vec<CstFunctionStatement>,
    constants: Vec<CstConstantStatement>,
}

impl CstFunctionStatement {

    pub(crate) fn new(
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

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn emits(&self) -> &Vec<CstEmitStatement> {
        &self.emits
    }

    pub(crate) fn constants(&self) -> &Vec<CstConstantStatement> {
        &self.constants
    }

    pub(crate) fn functions(&self) -> &Vec<CstFunctionStatement> {
        &self.functions
    }
}