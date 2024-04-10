use crate::ast::{AstNode, AstNodeType};
use crate::encoding;

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
    pub(crate) atoms: Vec<CstAtom>,
}

#[derive(Debug)]
pub(crate) struct CstStatementEmit {
    pub(crate) atoms: Vec<CstAtom>,
}

#[derive(Debug)]
pub(crate) enum CstStatement {
    Emit(CstStatementEmit),
    Const(CstStatementConst),
}

#[derive(Debug, Clone)]
pub(crate) enum CstAtom {
    Bytes { value: Vec<u8> },
    Utf8 { value: String },
    Const { name: String },
    Fn { name: String, params: Vec<CstAtom> },
}

impl CstAtom {

    pub(crate) fn len(&self) -> usize {
        match self {
            CstAtom::Bytes { value } => value.len(),
            CstAtom::Utf8 { value } => value.len(),
            _ => panic!("can't get len of unresolved atom")
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
            return CstStatement::Const(CstStatementConst { name, atoms });
        }
        AstNodeType::IGNORED => {
            return CstStatement::Emit(CstStatementEmit { atoms: Vec::new() });
        }
        _ => panic!("Unexpected node type: {:?}", ast_node.node_type),
    }
}

fn parse_cst_atom(node: AstNode) -> CstAtom {
    let node_value = node.value;

    match node.node_type {
        crate::ast::AstNodeType::AtomUtf8 => {
            return CstAtom::Utf8 {
                value: node_value.unwrap(),
            };
        }
        crate::ast::AstNodeType::AtomHex => {
            return CstAtom::Bytes {
                value: encoding::decode_byte(node_value.unwrap()).unwrap(),
            };
        }
        crate::ast::AstNodeType::AtomConst => {
            return CstAtom::Const {
                name: node_value.unwrap(),
            };
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
            return CstAtom::Fn { name, params };
        }
        _ => panic!("Unexpected node type: {:?}", node.node_type),
    }
}
