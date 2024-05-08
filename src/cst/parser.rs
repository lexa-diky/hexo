use std::path::PathBuf;
use crate::ast::{AstNode, AstNodeType, AstParserError};
use crate::cst::{CstAtom, CstConstantStatement, CstEmitStatement, CstFunctionStatement};
use crate::cst::node::CstFile;
use crate::encoding;

#[derive(Debug)]
pub(crate) enum CstParserError {
    UnexpectedNode { expected: Vec<AstNodeType>, actual: AstNodeType },
    MalformedNodeValue { message: String },
    MissingContent { node_type: AstNodeType },
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
            _ => panic!("Unexpected node type: {:?}", child.node_type)
        }
    }

    return Ok(
        CstEmitStatement {
            atoms
        }
    );
}

fn parse_atom_hex_into(node: &AstNode, buf: &mut Vec<CstAtom>) -> Result<(), CstParserError> {
    let content = node.clone().content
        .ok_or(CstParserError::MissingContent { node_type: AstNodeType::AtomHex })?;

    let bytes = encoding::decode_byte_ref(&content)
        .map_err(|x| CstParserError::MalformedNodeValue {
            message: format!("can't parse bytes {}", content)
        }
        )?;

    for byte in bytes {
        buf.push(CstAtom::Hex(byte))
    }
    return Ok(());
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