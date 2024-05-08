use std::path::PathBuf;

use crate::ast::{AstNode, AstNodeType};
use crate::cst::{CstAtom, CstConstantStatement, CstEmitStatement, CstFunctionStatement};
use crate::cst::node::CstFile;
use crate::encoding;

#[derive(Debug)]
pub(crate) enum CstParserError {
    UnexpectedNode { expected: Vec<AstNodeType>, actual: AstNodeType },
    MalformedNodeValue { message: String },
    MissingContent { node_type: AstNodeType },
    UnexpectedChildren { node_type: AstNodeType, children: Vec<AstNodeType> },
    DuplicateNode,
}

pub(crate) struct CstParser {}

impl CstParser {
    pub(crate) fn new() -> CstParser {
        CstParser {}
    }

    pub(crate) fn parse(&self, path: PathBuf, ast_root: AstNode) -> Result<CstFile, CstParserError> {
        return parse_file(path, &ast_root);
    }
}

const MAIN_FUNCTION_NAME: &str = "main";

fn parse_file(path: PathBuf, node: &AstNode) -> Result<CstFile, CstParserError> {
    guard_node_type(node, AstNodeType::File)?;
    let (emits, functions, constants) = parse_function_body(node)?;

    return Ok(
        CstFile {
            path: path,
            main: CstFunctionStatement {
                name: MAIN_FUNCTION_NAME.to_string(),
                params: Vec::new(),
                emits,
                functions,
                constants,
            },
        });
}

type BodyParsingResult = (
    Vec<CstEmitStatement>,
    Vec<CstFunctionStatement>,
    Vec<CstConstantStatement>
);

fn parse_function_body(node: &AstNode) -> Result<BodyParsingResult, CstParserError> {
    let mut emits = Vec::new();
    let mut functions = Vec::new();
    let mut constants = Vec::new();

    for child in &node.children {
        match child.node_type {
            AstNodeType::StatementConst => {}
            AstNodeType::StatementEmit => emits.push(parse_emit_statement(child)?),
            AstNodeType::StatementFn => {}
            _ => return Err(CstParserError::UnexpectedNode {
                actual: child.node_type,
                expected: vec![
                    AstNodeType::StatementConst,
                    AstNodeType::StatementEmit,
                    AstNodeType::StatementFn,
                ],
            }),
        }
    }

    Ok((emits, functions, constants))
}

fn parse_emit_statement(node: &AstNode) -> Result<CstEmitStatement, CstParserError> {
    guard_node_type(node, AstNodeType::StatementEmit)?;
    let mut atoms = Vec::new();

    for child in &node.children {
        match child.node_type {
            AstNodeType::AtomHex => parse_atom_hex_into(child, &mut atoms)?,
            AstNodeType::AtomUtf8 => parse_atom_utf8_into(child, &mut atoms)?,
            AstNodeType::AtomBaseNumber => parse_atom_base_num_into(child, &mut atoms)?,
            AstNodeType::AtomConst => parse_atom_constant_into(child, &mut atoms)?,
            _ => return Err(CstParserError::UnexpectedNode {
                actual: child.node_type,
                expected: vec![
                    AstNodeType::AtomHex,
                    AstNodeType::AtomUtf8,
                    AstNodeType::AtomBaseNumber,
                    AstNodeType::AtomConst
                ],
            })
        }
    }

    return Ok(
        CstEmitStatement {
            atoms
        }
    );
}

fn parse_atom_constant_into(node: &AstNode, buf: &mut Vec<CstAtom>) -> Result<(), CstParserError> {
    guard_node_type(node, AstNodeType::AtomConst)?;
    let content = parse_value_of(node)?;
    let atom = CstAtom::Constant { name: content };

    buf.push(atom);

    Ok(())
}

fn parse_atom_hex_into(node: &AstNode, buf: &mut Vec<CstAtom>) -> Result<(), CstParserError> {
    guard_node_type(node, AstNodeType::AtomHex)?;

    let content = node.clone().content
        .ok_or(CstParserError::MissingContent { node_type: AstNodeType::AtomHex })?;

    let bytes = encoding::decode_byte_ref(&content)
        .map_err(|x| CstParserError::MalformedNodeValue {
            message: format!("can't parse bytes {}", content)
        })?;

    for byte in bytes {
        buf.push(CstAtom::Hex(byte))
    }
    return Ok(());
}

fn parse_atom_utf8_into(node: &AstNode, buf: &mut Vec<CstAtom>) -> Result<(), CstParserError> {
    guard_node_type(node, AstNodeType::AtomUtf8)?;

    let content = node.clone().content
        .ok_or(CstParserError::MissingContent { node_type: AstNodeType::AtomUtf8 })?;

    buf.push(CstAtom::String(content));

    return Ok(());
}

fn parse_atom_base_num_into(node: &AstNode, buf: &mut Vec<CstAtom>) -> Result<(), CstParserError> {
    guard_node_type(node, AstNodeType::AtomBaseNumber)?;
    let mut base = None;
    let mut value = None;

    for child in &node.children {
        match child.node_type {
            AstNodeType::AtomBaseNumberBase => {
                guard_empty(base)?;
                base = Some(parse_value_of(child)?)
            }
            AstNodeType::AtomBaseNumberValue => {
                guard_empty(value)?;
                value = Some(parse_value_of(child)?)
            }
            _ => return Err(CstParserError::UnexpectedNode {
                actual: child.node_type,
                expected: vec![
                    AstNodeType::AtomBaseNumberBase,
                    AstNodeType::AtomBaseNumberValue,
                ],
            })
        }
    }

    let base_value_str = base.ok_or(CstParserError::MissingContent { node_type: AstNodeType::AtomBaseNumberBase })?;
    let base_value = u32::from_str_radix(base_value_str.as_str(), 10)
        .map_err(|_| CstParserError::MalformedNodeValue {
            message: format!("can't parse base {}", base_value_str)
        })?;

    let value_str = value.ok_or(CstParserError::MissingContent { node_type: AstNodeType::AtomBaseNumberValue })?;

    buf.push(
        CstAtom::Number {
            value: u32::from_str_radix(value_str.as_str(), base_value)
                .map_err(|_| CstParserError::MalformedNodeValue {
                    message: format!("can't parse number {}", value_str)
                })?
        }
    );

    return Ok(());
}

fn parse_value_of(node: &AstNode) -> Result<String, CstParserError> {
    if !node.children.is_empty() {
        return Err(CstParserError::UnexpectedChildren {
            node_type: node.node_type,
            children: node.children.iter().map(|x| x.node_type).collect(),
        });
    }

    return node.clone().content
        .ok_or(CstParserError::MissingContent { node_type: node.node_type });
}

fn guard_node_type(node: &AstNode, expected_type: AstNodeType) -> Result<(), CstParserError> {
    if node.node_type != expected_type {
        return Err(
            CstParserError::UnexpectedNode {
                actual: node.node_type,
                expected: vec![expected_type],
            }
        );
    }

    return Ok(());
}

fn guard_empty<T>(option: Option<T>) -> Result<(), CstParserError> {
    if option.is_some() {
        return Err(CstParserError::DuplicateNode);
    }

    return Ok(());
}