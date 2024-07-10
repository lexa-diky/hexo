use std::path::{Path, PathBuf};

use crate::compiler::ast::{AstNode, AstNodeType};
use crate::compiler::cst::Error;
use crate::compiler::cst::CstFile;
use crate::compiler::cst::{
    CstActualParameter, CstAtom, CstConstantStatement, CstEmitStatement, CstFunctionStatement,
};
use crate::match_ast;

#[derive(Default)]
pub(crate) struct CstParser {}

impl CstParser {
    pub(crate) fn parse(&self, path: &Path, ast_root: AstNode) -> Result<CstFile, Error> {
        parse_file(path, &ast_root)
    }
}

const MAIN_FUNCTION_NAME: &str = "main";

fn parse_file(path: &Path, node: &AstNode) -> Result<CstFile, Error> {
    guard_node_type(node, AstNodeType::File)?;
    let (emits, functions, constants) = parse_function_body(node)?;

    Ok(
        CstFile::new(
            path.to_path_buf(),
            CstFunctionStatement::new(
                MAIN_FUNCTION_NAME.to_string(),
                emits,
                functions,
                constants,
            ),
        )
    )
}

type BodyParsingResult = (
    Vec<CstEmitStatement>,
    Vec<CstFunctionStatement>,
    Vec<CstConstantStatement>,
);

fn parse_function_body(node: &AstNode) -> Result<BodyParsingResult, Error> {
    let mut emits = Vec::new();
    let mut functions = Vec::new();
    let mut constants = Vec::new();

    for child in node.children() {
        match child.node_type() {
            AstNodeType::StatementConst => constants.push(parse_constant(child)?),
            AstNodeType::StatementEmit => emits.push(parse_emit_statement(child)?),
            AstNodeType::StatementFn => functions.push(parse_function(child)?),
            _ => {
                return Err(Error::UnexpectedNode {
                    actual: child.node_type(),
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

fn parse_constant(node: &AstNode) -> Result<CstConstantStatement, Error> {
    guard_node_type(node, AstNodeType::StatementConst)?;
    let mut atom_buff = Vec::new();
    let mut name = None;

    for child in node.children() {
        match child.node_type() {
            AstNodeType::StatementConstName => {
                name = Some(parse_value_of(child)?);
            }
            _ => parse_atom_into(child, &mut atom_buff)?,
        }
    }

    Ok(
        CstConstantStatement::new(
            name
                .ok_or(Error::MissingContent {
                    node_type: AstNodeType::StatementConstName,
                })?
                .to_string(),
            atom_buff,
        )
    )
}

fn parse_function(node: &AstNode) -> Result<CstFunctionStatement, Error> {
    let mut name = None;
    let mut emits = None;
    let mut functions = None;
    let mut constants = None;

    for child in node.children() {
        match child.node_type() {
            AstNodeType::StatementFnName => {
                name = Some(parse_value_of(child)?);
            }
            AstNodeType::StatementFnBody => {
                let (emits_r, functions_r, constants_r) = parse_function_body(child)?;
                emits = Some(emits_r);
                functions = Some(functions_r);
                constants = Some(constants_r);
            }
            _ => {
                return Err(Error::UnexpectedNode {
                    actual: child.node_type(),
                    expected: vec![AstNodeType::StatementFnName, AstNodeType::StatementFnBody],
                })
            }
        }
    }

    Ok(
        CstFunctionStatement::new(
            name.ok_or(Error::MissingContent {
                node_type: AstNodeType::StatementFnName,
            })?,
            emits.unwrap_or(Vec::new()),
            functions.unwrap_or(Vec::new()),
            constants.unwrap_or(Vec::new()),
        )
    )
}

fn parse_emit_statement(node: &AstNode) -> Result<CstEmitStatement, Error> {
    guard_node_type(node, AstNodeType::StatementEmit)?;
    let mut atoms = Vec::new();

    for child in node.children() {
        parse_atom_into(child, &mut atoms)?
    }

    Ok(CstEmitStatement::new(atoms))
}

fn parse_atom_into(node: &AstNode, buff: &mut Vec<CstAtom>) -> Result<(), Error> {
    match node.node_type() {
        AstNodeType::AtomHex => parse_atom_hex_into(node, buff)?,
        AstNodeType::AtomUtf8 => parse_atom_utf8_into(node, buff)?,
        AstNodeType::AtomBaseNumber => parse_atom_base_num_into(node, buff)?,
        AstNodeType::AtomConst => parse_atom_constant_into(node, buff)?,
        AstNodeType::AtomFn => parse_atom_function_into(node, buff)?,
        _ => {
            return Err(Error::UnexpectedNode {
                actual: node.node_type(),
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

fn parse_atom_constant_into(node: &AstNode, buf: &mut Vec<CstAtom>) -> Result<(), Error> {
    guard_node_type(node, AstNodeType::AtomConst)?;
    let content = parse_value_of(node)?;
    let atom = CstAtom::Constant { name: content };

    buf.push(atom);

    Ok(())
}

fn parse_atom_function_into(node: &AstNode, buf: &mut Vec<CstAtom>) -> Result<(), Error> {
    guard_node_type(node, AstNodeType::AtomFn)?;
    let mut name = None;
    let mut params = None;

    for child in node.children() {
        match child.node_type() {
            AstNodeType::AtomFnName => {
                guard_empty(name)?;
                name = Some(parse_value_of(child)?);
            }
            AstNodeType::AtomFnParams => params = Some(parse_atom_fn_params(child)?),
            _ => {
                return Err(Error::UnexpectedNode {
                    actual: child.node_type(),
                    expected: vec![AstNodeType::AtomFnName, AstNodeType::AtomFnParams],
                })
            }
        }
    }

    let name_value = name.ok_or(Error::MissingContent {
        node_type: AstNodeType::AtomFnName,
    })?;

    let params_value = params.unwrap_or(Vec::new());

    buf.push(CstAtom::Function {
        name: name_value,
        params: params_value,
    });

    Ok(())
}

fn parse_atom_fn_params(node: &AstNode) -> Result<Vec<CstActualParameter>, Error> {
    guard_node_type(node, AstNodeType::AtomFnParams)?;

    let mut buf = Vec::new();

    for (param_counter, child) in node.children().iter().enumerate() {
        guard_node_type(child, AstNodeType::AtomFnParam)?;
        let mut value = Vec::new();
        let mut name = None;

        for p_child in child.children() {
            match p_child.node_type() {
                AstNodeType::AtomFnParamValue => {
                    for value_node in p_child.children() {
                        parse_atom_into(value_node, &mut value)?;
                    }
                }
                AstNodeType::AtomFnParamIdentifier => {
                    name = Some(parse_value_of(p_child)?);
                }
                _ => {
                    return Err(Error::UnexpectedNode {
                        actual: p_child.node_type(),
                        expected: vec![AstNodeType::AtomFnParamValue],
                    })
                }
            }
        }

        buf.push(CstActualParameter::new(
            name.unwrap_or(param_counter.to_string()),
            value,
        ));
    }

    Ok(buf)
}

fn parse_atom_hex_into(node: &AstNode, buf: &mut Vec<CstAtom>) -> Result<(), Error> {
    guard_node_type(node, AstNodeType::AtomHex)?;

    let content = node.content().ok_or(Error::MissingContent {
        node_type: AstNodeType::AtomHex,
    })?;

    let bytes = decode_bytes_from_string(content.as_str())?;
    for byte in bytes {
        buf.push(CstAtom::Hex(byte))
    }
    Ok(())
}

pub(crate) fn decode_bytes_from_string(s: &str) -> Result<Vec<u8>, Error> {
    (0..s.len()).step_by(2)
        .map(|i| {
            if s.len() < 2 {
                return Err(
                    Error::MalformedNodeValue {
                        message: format!("can't parse bytes {}", s),
                    }
                );
            }
            u8::from_str_radix(&s[i..i + 2], 16)
                .map_err(|_| Error::MalformedNodeValue {
                    message: format!("can't parse bytes {}", s),
                })
        })
        .collect()
}

fn parse_atom_utf8_into(node: &AstNode, buf: &mut Vec<CstAtom>) -> Result<(), Error> {
    guard_node_type(node, AstNodeType::AtomUtf8)?;
    let content = parse_value_of(node)?;
    buf.push(CstAtom::String(content));

    Ok(())
}

fn parse_atom_base_num_into(node: &AstNode, buf: &mut Vec<CstAtom>) -> Result<(), Error> {
    fn parse_number_base(base: String) -> Result<u32, Error> {
        let base_value = base.parse()
            .map_err(|_| Error::MalformedNodeValue {
                message: format!("can't parse base {}", base),
            })?;
        Ok(base_value)
    }

    match_ast!(
        node => AtomBaseNumber,
        AtomBaseNumberBase => base | parse_number_base
        AtomBaseNumberValue => value | Ok
    );

    buf.push(
        CstAtom::Number(
            u32::from_str_radix(value.as_str(), base).map_err(|_| {
                Error::MalformedNodeValue {
                    message: format!("can't parse number {}", value),
                }
            })?,
        )
    );

    Ok(())
}

fn parse_value_of(node: &AstNode) -> Result<String, Error> {
    if !node.children().is_empty() {
        return Err(Error::UnexpectedChildren {
            node_type: node.node_type(),
            children: node.children().iter().map(|x| x.node_type()).collect(),
        });
    }

    node.content().ok_or(Error::MissingContent {
        node_type: node.node_type(),
    }).cloned()
}

fn guard_node_type(node: &AstNode, expected_type: AstNodeType) -> Result<(), Error> {
    if node.node_type() != expected_type {
        return Err(Error::UnexpectedNode {
            actual: node.node_type(),
            expected: vec![expected_type],
        });
    }

    Ok(())
}

fn guard_empty<T>(option: Option<T>) -> Result<(), Error> {
    if option.is_some() {
        return Err(Error::DuplicateNode);
    }

    Ok(())
}
