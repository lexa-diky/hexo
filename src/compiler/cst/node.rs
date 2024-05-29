use std::path::PathBuf;

#[derive(Clone, Debug)]
pub(crate) struct CstFile {
    pub(crate) path: PathBuf,
    pub(crate) main: CstFunctionStatement,
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
pub(crate) struct CstFunctionParameter {
    pub(crate) name: String,
}

#[derive(Clone, Debug)]
pub(crate) struct CstActualParameter {
    pub(crate) name: String,
    pub(crate) value: CstAtomVec,
}

pub(crate) type CstAtomVec = Vec<CstAtom>;

#[derive(Clone, Debug)]
pub(crate) struct CstEmitStatement {
    pub(crate) atoms: CstAtomVec,
}

#[derive(Clone, Debug)]
pub(crate) struct CstConstantStatement {
    pub(crate) name: String,
    pub(crate) atoms: CstAtomVec,
}

#[derive(Clone, Debug)]
pub(crate) struct CstFunctionStatement {
    pub(crate) name: String,
    pub(crate) params: Vec<CstFunctionParameter>,
    pub(crate) emits: Vec<CstEmitStatement>,
    pub(crate) functions: Vec<CstFunctionStatement>,
    pub(crate) constants: Vec<CstConstantStatement>,
}
