use crate::ast::{AstNode, AstNodeType};
use crate::encoding;
use crate::encoding::to_shrunk_bytes;

#[derive(Debug, Clone)]
pub(crate) struct CstAtomStrip(Vec<CstAtom>);

impl CstAtomStrip {
    pub(crate) fn from(atoms: Vec<CstAtom>) -> CstAtomStrip {
        CstAtomStrip(atoms)
    }

    pub(crate) fn empty() -> CstAtomStrip {
        CstAtomStrip(Vec::new())
    }

    pub(crate) fn iter(&self) -> std::slice::Iter<CstAtom> {
        self.0.iter()
    }

    pub(crate) fn to_vec(&self) -> Vec<CstAtom> {
        self.0.clone()
    }

    pub(crate) fn push(&mut self, atom: CstAtom) {
        self.0.push(atom);
    }

    pub(crate) fn extend(&mut self, atoms: CstAtomStrip) {
        self.0.extend(atoms.0);
    }
}

impl FromIterator<CstAtom> for CstAtomStrip {
    fn from_iter<T: IntoIterator<Item=CstAtom>>(iter: T) -> Self {
        CstAtomStrip(iter.into_iter().collect())
    }
}


#[derive(Debug)]
pub(crate) struct CstFile {
    pub(crate) file_name: String,
    pub(crate) statements: Vec<CstStatement>,
}

impl CstFile {
    pub(crate) fn constants(&self) -> Vec<&CstStatementConst> {
        self.statements
            .iter()
            .filter_map(|s| match s {
                CstStatement::Const(it) => Some(it),
                _ => None,
            })
            .collect()
    }

    pub(crate) fn emits(&self) -> Vec<&CstStatementEmit> {
        self.statements
            .iter()
            .filter_map(|s| match s {
                CstStatement::Emit(it) => Some(it),
                _ => None,
            })
            .collect()
    }
}

#[derive(Debug)]
pub(crate) struct CstStatementConst {
    pub(crate) name: String,
    pub(crate) atoms: CstAtomStrip,
}

#[derive(Debug)]
pub(crate) struct CstStatementEmit {
    pub(crate) atoms: CstAtomStrip,
}

#[derive(Debug)]
pub(crate) enum CstStatement {
    Emit(CstStatementEmit),
    Const(CstStatementConst),
}

#[derive(Debug, Clone)]
pub(crate) enum CstAtomUnresolved {
    Const {
        name: String,
    },
    Fn {
        name: String,
        params_flatten: Vec<CstAtom>,
    },
}

#[derive(Debug, Clone)]
pub(crate) enum CstAtom {
    Resolved { value: Vec<u8> },
    Unresolved(CstAtomUnresolved),
}

impl CstAtom {
    pub(crate) fn len(&self) -> usize {
        match self {
            CstAtom::Resolved { value } => value.len(),
            _ => panic!("can't get len of unresolved atom"),
        }
    }
}

pub(crate) fn parse_cst(ast_node: AstNode) -> CstFile {
    assert_eq!(ast_node.node_type, AstNodeType::File);

    return CstFile {
        file_name: ast_node.value.unwrap(),
        statements: ast_node
            .children
            .into_iter()
            .map(parse_cst_statement)
            .collect(),
    };
}

fn parse_cst_statement(ast_node: AstNode) -> CstStatement {
    match ast_node.node_type {
        AstNodeType::StatementEmit => {
            return CstStatement::Emit(CstStatementEmit {
                atoms: ast_node.children.into_iter().map(parse_cst_atom).collect(),
            });
        }
        AstNodeType::StatementConst => {
            let mut name = String::new();
            let mut atoms = Vec::new();
            for child in ast_node.children {
                match child.node_type {
                    AstNodeType::StatementConstName => {
                        name = child.value.unwrap();
                    }
                    _ => {
                        atoms.push(parse_cst_atom(child));
                    }
                }
            }
            return CstStatement::Const(
                CstStatementConst {
                    name: name,
                    atoms: CstAtomStrip::from(atoms),
                }
            );
        }
        AstNodeType::IGNORED => {
            return CstStatement::Emit(CstStatementEmit { atoms: CstAtomStrip::empty() });
        }
        _ => panic!("Unexpected node type: {:?}", ast_node.node_type),
    }
}

fn parse_cst_atom(node: AstNode) -> CstAtom {
    let node_value = node.value;

    match node.node_type {
        AstNodeType::AtomUtf8 => {
            return CstAtom::Resolved {
                value: node_value.unwrap().into_bytes(),
            };
        }
        AstNodeType::AtomHex => {
            return CstAtom::Resolved {
                value: encoding::decode_byte(node_value.unwrap()).unwrap(),
            };
        }
        AstNodeType::AtomConst => {
            return CstAtom::Unresolved(CstAtomUnresolved::Const {
                name: node_value.unwrap(),
            });
        }
        AstNodeType::AtomFn => {
            let mut name = String::new();
            let mut params = Vec::new();
            for child in node.children {
                match child.node_type {
                    AstNodeType::AtomFnName => {
                        name = child.value.unwrap();
                    }
                    AstNodeType::StatementConstParams => {
                        child.children.into_iter().for_each(|param| {
                            params.push(parse_cst_atom(param));
                        });
                    }
                    _ => {}
                }
            }
            return CstAtom::Unresolved(CstAtomUnresolved::Fn {
                name,
                params_flatten: params,
            });
        }
        AstNodeType::AtomBaseNumber => {
            let mut base = 10;
            let mut value = String::new();
            for child in node.children {
                match child.node_type {
                    AstNodeType::AtomBaseNumberBase => {
                        base = child.value.unwrap().parse().unwrap();
                    }
                    AstNodeType::AtomBaseNumberValue => {
                        value = child.value.unwrap();
                    }
                    _ => {}
                }
            }
            let value = u32::from_str_radix(value.as_str(), base).unwrap();

            return CstAtom::Resolved {
                value: to_shrunk_bytes(value),
            };
        }
        _ => panic!("Unexpected node type: {:?}", node.node_type),
    }
}
