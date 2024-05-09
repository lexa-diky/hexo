use crate::compiler::ast::{AstNode, AstNodeType};
use crate::cst_legacy::CstParseError::NodeValueMissing;
use crate::encoding_legacy;
use crate::encoding_legacy::to_shrunk_bytes;

#[derive(Debug, Clone)]
pub(crate) struct CstAtomStrip(Vec<CstAtom>);

impl CstAtomStrip {
    pub(crate) fn new(atoms: Vec<CstAtom>) -> CstAtomStrip {
        CstAtomStrip(atoms)
    }

    pub(crate) fn empty() -> CstAtomStrip {
        CstAtomStrip(Vec::new())
    }

    pub(crate) fn iter(&self) -> std::slice::Iter<CstAtom> {
        self.0.iter()
    }

    pub(crate) fn push(&mut self, atom: CstAtom) {
        self.0.push(atom);
    }

    pub(crate) fn extend(&mut self, atoms: CstAtomStrip) {
        self.0.extend(atoms.0);
    }

    pub(crate) fn len(&self) -> Result<usize, CstParseError> {
        let mut acc = 0;
        for atom in &self.0 {
            acc += match atom {
                CstAtom::Resolved { value } => value.len(),
                CstAtom::Unresolved(_) => return Err(CstParseError::UnresolvedAtom),
            };
        }

        return Ok(acc);
    }

    pub(crate) fn as_usize(&self) -> usize {
        let data = self.clamp_vec_u8();
        data[0] as usize // TODO not correct behavior
    }

    pub(crate) fn clamp_vec_u8(&self) -> Vec<u8> {
        let mut buff = Vec::new();
        self.0.iter().for_each(|atom| {
            if let CstAtom::Resolved { value } = atom {
                buff.extend(value.iter().cloned());
            } else {
                panic!("can't get usize of unresolved atom")
            }
        });

        buff
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

    pub(crate) fn functions(&self) -> Vec<&CstStatementFn> {
        self.statements
            .iter()
            .filter_map(|s| match s {
                CstStatement::Fn(it) => Some(it),
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

#[derive(Debug, Clone)]
pub(crate) struct CstStatementEmit {
    pub(crate) atoms: CstAtomStrip,
}

#[derive(Debug)]
pub(crate) struct CstStatementFn {
    pub(crate) name: String,
    pub(crate) statements: Vec<CstStatementEmit>,
}

#[derive(Debug)]
pub(crate) enum CstStatement {
    Emit(CstStatementEmit),
    Const(CstStatementConst),
    Fn(CstStatementFn),
}

#[derive(Debug, Clone)]
pub(crate) struct CstFunctionParameter {
    pub(crate) params: Vec<CstAtom>,
}

impl CstFunctionParameter {
    pub(crate) fn new() -> CstFunctionParameter {
        CstFunctionParameter { params: Vec::new() }
    }

    pub(crate) fn push(&mut self, atom: CstAtom) {
        self.params.push(atom);
    }
}

#[derive(Debug, Clone)]
pub(crate) enum CstAtomUnresolved {
    Const {
        name: String,
    },
    Fn {
        name: String,
        params: Vec<CstFunctionParameter>,
    },
}

#[derive(Debug, Clone)]
pub(crate) enum CstAtom {
    Resolved { value: Vec<u8> },
    Unresolved(CstAtomUnresolved),
}

#[derive(Debug)]
pub(crate) enum CstParseError {
    NodeValueMissing(AstNodeType),
    CstValueNotFound,
    CstChildNotFound,
    Unexpected { message: &'static str },
    UnresolvedAtom,
}

pub(crate) fn parse_cst(ast_node: AstNode) -> Result<CstFile, CstParseError> {
    assert_eq!(ast_node.node_type, AstNodeType::File);

    return Ok(
        CstFile {
            file_name: "unknown".to_string(), // TODO remove unknown
            statements: ast_node
                .children
                .into_iter()
                .map(parse_cst_statement)
                .collect(),
        }
    );
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
                        name = child.content.unwrap();
                    }
                    _ => {
                        atoms.push(parse_cst_atom(child));
                    }
                }
            }
            return CstStatement::Const(CstStatementConst {
                name: name,
                atoms: CstAtomStrip::new(atoms),
            });
        }
        AstNodeType::StatementFn => {
            let name = lookup_value(AstNodeType::StatementFnName, &ast_node).unwrap();
            let body = lookup_child(AstNodeType::StatementFnBody, &ast_node).unwrap();
            return CstStatement::Fn(CstStatementFn {
                name: name,
                statements: body
                    .children
                    .into_iter()
                    .map(parse_cst_statement)
                    .map(|statement| match statement {
                        CstStatement::Emit(inner) => inner,
                        _ => panic!("Unexpected statement type: {:?}", statement),
                    })
                    .collect(),
            });
        }
        _ => panic!("Unexpected node type: {:?}", ast_node.node_type),
    }
}

fn lookup_value(node_type: AstNodeType, in_node: &AstNode) -> Result<String, CstParseError> {
    for child in &in_node.children {
        if child.node_type == node_type {
            return <Option<String> as Clone>::clone(&child.content)
                .ok_or(CstParseError::Unexpected { message: "Can't clone child value" });
        }
    }
    return Err(CstParseError::CstValueNotFound);
}

fn lookup_child(node_type: AstNodeType, in_node: &AstNode) -> Result<AstNode, CstParseError> {
    for child in &in_node.children {
        if child.node_type == node_type {
            return Ok(child.clone());
        }
    }
    return Err(CstParseError::CstChildNotFound);
}

fn parse_cst_atom(node: AstNode) -> CstAtom {
    let node_value = node.content;

    match node.node_type {
        AstNodeType::AtomUtf8 => {
            return CstAtom::Resolved {
                value: node_value.unwrap().into_bytes(),
            };
        }
        AstNodeType::AtomHex => {
            return CstAtom::Resolved {
                value: encoding_legacy::decode_byte(node_value.unwrap()).unwrap(),
            };
        }
        AstNodeType::AtomConst => {
            return CstAtom::Unresolved(CstAtomUnresolved::Const {
                name: node_value.unwrap(),
            });
        }
        AstNodeType::AtomFn => {
            let mut name = String::new();
            let mut parameter_buff: Vec<CstFunctionParameter> = Vec::new();

            for child in node.children {
                match child.node_type {
                    AstNodeType::AtomFnName => {
                        name = child.content.unwrap();
                    }
                    AstNodeType::AtomFnParams => child.children.iter().for_each(|param| {
                        let mut parameter = CstFunctionParameter::new();

                        param.clone().children.into_iter().for_each(|atom| {
                            let cst_atom = parse_cst_atom(atom);
                            parameter.push(cst_atom);
                        });

                        parameter_buff.push(parameter);
                    }),
                    _ => {}
                }
            }
            return CstAtom::Unresolved(CstAtomUnresolved::Fn {
                name,
                params: parameter_buff,
            });
        }
        AstNodeType::AtomBaseNumber => {
            let mut base = 10;
            let mut value = String::new();
            for child in node.children {
                match child.node_type {
                    AstNodeType::AtomBaseNumberBase => {
                        base = child.content.unwrap().parse().unwrap();
                    }
                    AstNodeType::AtomBaseNumberValue => {
                        value = child.content.unwrap();
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
