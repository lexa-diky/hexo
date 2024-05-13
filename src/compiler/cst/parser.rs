use std::path::PathBuf;

use crate::compiler::ast::{AstNode, AstNodeType};
use crate::compiler::cst::node::CstFile;
use crate::compiler::cst::{
    CstActualParameter, CstAtom, CstConstantStatement, CstEmitStatement, CstFunctionStatement,
};
use crate::compiler::util::encoding::decode_bytes_from_string;
use crate::encoding_legacy;

#[derive(Debug)]
pub(crate) enum CstParserError {
    UnexpectedNode {
        expected: Vec<AstNodeType>,
        actual: AstNodeType,
    },
    MalformedNodeValue {
        message: String,
    },
    MissingContent {
        node_type: AstNodeType,
    },
    UnexpectedChildren {
        node_type: AstNodeType,
        children: Vec<AstNodeType>,
    },
    DuplicateNode,
}

pub(crate) struct CstParser {}

impl CstParser {
    pub(crate) fn new() -> CstParser {
        CstParser {}
    }

    pub(crate) fn parse(
        &self,
        path: PathBuf,
        ast_root: AstNode,
    ) -> Result<CstFile, CstParserError> {
        return parse_file(path, &ast_root);
    }
}

const MAIN_FUNCTION_NAME: &str = "main";

fn parse_file(path: PathBuf, node: &AstNode) -> Result<CstFile, CstParserError> {
    guard_node_type(node, AstNodeType::File)?;
    let (emits, functions, constants) = parse_function_body(node)?;

    return Ok(CstFile {
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
    Vec<CstConstantStatement>,
);

fn parse_function_body(node: &AstNode) -> Result<BodyParsingResult, CstParserError> {
    let mut emits = Vec::new();
    let mut functions = Vec::new();
    let mut constants = Vec::new();

    for child in &node.children {
        match child.node_type {
            AstNodeType::StatementConst => constants.push(parse_constant(child)?),
            AstNodeType::StatementEmit => emits.push(parse_emit_statement(child)?),
            AstNodeType::StatementFn => functions.push(parse_function(child)?),
            _ => {
                return Err(CstParserError::UnexpectedNode {
                    actual: child.node_type,
                    expected: vec![
                        AstNodeType::StatementConst,
                        AstNodeType::StatementEmit,
                        AstNodeType::StatementFn,
                    ],
                })
            }
        }
    }

    Ok((emits, functions, constants))
}

fn parse_constant(node: &AstNode) -> Result<CstConstantStatement, CstParserError> {
    guard_node_type(node, AstNodeType::StatementConst)?;
    let mut atom_buff = Vec::new();
    let mut name = None;

    for child in &node.children {
        match child.node_type {
            AstNodeType::StatementConstName => {
                name = Some(parse_value_of(child)?);
            }
            _ => parse_atom_into(child, &mut atom_buff)?,
        }
    }

    Ok(CstConstantStatement {
        name: name
            .ok_or(CstParserError::MissingContent {
                node_type: AstNodeType::StatementConstName,
            })?
            .to_string(),
        atoms: atom_buff,
    })
}

fn parse_function(node: &AstNode) -> Result<CstFunctionStatement, CstParserError> {
    let mut emits = None;
    let mut functions = None;
    let mut constants = None;

    for child in &node.children {
        match child.node_type {
            AstNodeType::StatementFnName => {}
            AstNodeType::StatementFnBody => {
                let (emits_r, functions_r, constants_r) = parse_function_body(child)?;
                emits = Some(emits_r);
                functions = Some(functions_r);
                constants = Some(constants_r);
            }
            _ => {
                return Err(CstParserError::UnexpectedNode {
                    actual: child.node_type,
                    expected: vec![AstNodeType::StatementFnName, AstNodeType::StatementFnBody],
                })
            }
        }
    }

    return Ok(CstFunctionStatement {
        name: parse_value_of(&node.children[0])?,
        params: Vec::new(),
        emits: emits.unwrap_or(Vec::new()),
        functions: functions.unwrap_or(Vec::new()),
        constants: constants.unwrap_or(Vec::new()),
    });
}

fn parse_emit_statement(node: &AstNode) -> Result<CstEmitStatement, CstParserError> {
    guard_node_type(node, AstNodeType::StatementEmit)?;
    let mut atoms = Vec::new();

    for child in &node.children {
        parse_atom_into(child, &mut atoms)?
    }

    return Ok(CstEmitStatement { atoms: atoms });
}

fn parse_atom_into(node: &AstNode, buff: &mut Vec<CstAtom>) -> Result<(), CstParserError> {
    match node.node_type {
        AstNodeType::AtomHex => parse_atom_hex_into(node, buff)?,
        AstNodeType::AtomUtf8 => parse_atom_utf8_into(node, buff)?,
        AstNodeType::AtomBaseNumber => parse_atom_base_num_into(node, buff)?,
        AstNodeType::AtomConst => parse_atom_constant_into(node, buff)?,
        AstNodeType::AtomFn => parse_atom_function_into(node, buff)?,
        _ => {
            return Err(CstParserError::UnexpectedNode {
                actual: node.node_type,
                expected: vec![
                    AstNodeType::AtomHex,
                    AstNodeType::AtomUtf8,
                    AstNodeType::AtomBaseNumber,
                    AstNodeType::AtomConst,
                    AstNodeType::AtomFn,
                ],
            })
        }
    }

    Ok(())
}

fn parse_atom_constant_into(node: &AstNode, buf: &mut Vec<CstAtom>) -> Result<(), CstParserError> {
    guard_node_type(node, AstNodeType::AtomConst)?;
    let content = parse_value_of(node)?;
    let atom = CstAtom::Constant { name: content };

    buf.push(atom);

    Ok(())
}

fn parse_atom_function_into(node: &AstNode, buf: &mut Vec<CstAtom>) -> Result<(), CstParserError> {
    guard_node_type(node, AstNodeType::AtomFn)?;
    let mut name = None;
    let mut params = None;

    for child in &node.children {
        match child.node_type {
            AstNodeType::AtomFnName => {
                guard_empty(name)?;
                name = Some(parse_value_of(child)?);
            }
            AstNodeType::AtomFnParams => params = Some(parse_atom_fn_params(child)?),
            _ => {
                return Err(CstParserError::UnexpectedNode {
                    actual: child.node_type,
                    expected: vec![AstNodeType::AtomFnName, AstNodeType::AtomFnParams],
                })
            }
        }
    }

    let name_value = name.ok_or(CstParserError::MissingContent {
        node_type: AstNodeType::AtomFnName,
    })?;

    let params_value = params.ok_or(CstParserError::MissingContent {
        node_type: AstNodeType::AtomFnParams,
    })?;

    buf.push(CstAtom::Function {
        name: name_value,
        params: params_value,
    });

    Ok(())
}

fn parse_atom_fn_params(node: &AstNode) -> Result<Vec<CstActualParameter>, CstParserError> {
    guard_node_type(node, AstNodeType::AtomFnParams)?;

    let mut param_counter = 0;
    let mut buf = Vec::new();

    for child in &node.children {
        guard_node_type(child, AstNodeType::AtomFnParam)?;
        let mut value = Vec::new();

        for p_child in &child.children {
            parse_atom_into(p_child, &mut value)?;
        }

        buf.push(CstActualParameter {
            name: param_counter.to_string(),
            value: value,
        });
        param_counter += 1;
    }

    return Ok(buf);
}

fn parse_atom_hex_into(node: &AstNode, buf: &mut Vec<CstAtom>) -> Result<(), CstParserError> {
    guard_node_type(node, AstNodeType::AtomHex)?;

    let content = node.clone().content.ok_or(CstParserError::MissingContent {
        node_type: AstNodeType::AtomHex,
    })?;

    let bytes =
        decode_bytes_from_string(&content).map_err(|x| CstParserError::MalformedNodeValue {
            message: format!("can't parse bytes {}", content),
        })?;

    for byte in bytes {
        buf.push(CstAtom::Hex(byte))
    }
    return Ok(());
}

fn parse_atom_utf8_into(node: &AstNode, buf: &mut Vec<CstAtom>) -> Result<(), CstParserError> {
    guard_node_type(node, AstNodeType::AtomUtf8)?;

    let content = node.clone().content.ok_or(CstParserError::MissingContent {
        node_type: AstNodeType::AtomUtf8,
    })?;

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
            _ => {
                return Err(CstParserError::UnexpectedNode {
                    actual: child.node_type,
                    expected: vec![
                        AstNodeType::AtomBaseNumberBase,
                        AstNodeType::AtomBaseNumberValue,
                    ],
                })
            }
        }
    }

    let base_value_str = base.ok_or(CstParserError::MissingContent {
        node_type: AstNodeType::AtomBaseNumberBase,
    })?;
    let base_value = u32::from_str_radix(base_value_str.as_str(), 10).map_err(|_| {
        CstParserError::MalformedNodeValue {
            message: format!("can't parse base {}", base_value_str),
        }
    })?;

    let value_str = value.ok_or(CstParserError::MissingContent {
        node_type: AstNodeType::AtomBaseNumberValue,
    })?;

    buf.push(CstAtom::Number(
        u32::from_str_radix(value_str.as_str(), base_value).map_err(|_| {
            CstParserError::MalformedNodeValue {
                message: format!("can't parse number {}", value_str),
            }
        })?,
    ));

    return Ok(());
}

fn parse_value_of(node: &AstNode) -> Result<String, CstParserError> {
    if !node.children.is_empty() {
        return Err(CstParserError::UnexpectedChildren {
            node_type: node.node_type,
            children: node.children.iter().map(|x| x.node_type).collect(),
        });
    }

    return node.clone().content.ok_or(CstParserError::MissingContent {
        node_type: node.node_type,
    });
}

fn guard_node_type(node: &AstNode, expected_type: AstNodeType) -> Result<(), CstParserError> {
    if node.node_type != expected_type {
        return Err(CstParserError::UnexpectedNode {
            actual: node.node_type,
            expected: vec![expected_type],
        });
    }

    return Ok(());
}

fn guard_empty<T>(option: Option<T>) -> Result<(), CstParserError> {
    if option.is_some() {
        return Err(CstParserError::DuplicateNode);
    }

    return Ok(());
}
