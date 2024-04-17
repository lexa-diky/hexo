use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;
use std::fmt::Display;

type AstError = ();

#[derive(Debug, Clone)]
pub(crate) struct AstNode {
    pub(crate) node_type: AstNodeType,
    pub(crate) value: Option<String>,
    pub(crate) children: Vec<AstNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum AstNodeType {
    // Container
    File,

    StatementConst,
    StatementConstName,

    StatementEmit,

    StatementFn,
    StatementFnName,
    StatementFnBody,

    AtomUtf8,
    AtomHex,
    AtomConst,
    AtomFn,
    AtomFnParam,
    AtomFnParams,
    AtomBaseNumber,
    AtomBaseNumberBase,
    AtomBaseNumberValue,

    AtomFnName,

    // Fallback
    IGNORED,
    Unknown { rule_name: String },
}

impl AstNodeType {

    fn must_capture_value(&self) -> bool {
        match self {
            AstNodeType::AtomUtf8
            | AstNodeType::AtomHex
            | AstNodeType::AtomFnName
            | AstNodeType::StatementConstName
            | AstNodeType::AtomBaseNumberBase
            | AstNodeType::AtomBaseNumberValue
            | AstNodeType::StatementFnName
            | AstNodeType::AtomConst => true,
            _ => false,
        }
    }
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub(crate) struct HexoParser;

pub(crate) fn parse_ast(file_name: String, pairs: Pairs<Rule>) -> Result<AstNode, AstError> {
    let children: Result<Vec<AstNode>, _> = pairs.map(parse_ast_pair).collect();

    return Ok(
        AstNode {
            node_type: AstNodeType::File,
            value: Some(file_name),
            children: children?,
        }
    );
}

fn parse_ast_pair(p: Pair<Rule>) -> Result<AstNode, AstError> {
    let node_type = match p.as_rule() {
        Rule::atom_utf8 => AstNodeType::AtomUtf8,
        Rule::atom_hex => AstNodeType::AtomHex,
        Rule::atom_base_number => AstNodeType::AtomBaseNumber,
        Rule::atom_base_number_base => AstNodeType::AtomBaseNumberBase,
        Rule::atom_base_number_value => AstNodeType::AtomBaseNumberValue,
        Rule::atom_const => AstNodeType::AtomConst,

        Rule::atom_fn => AstNodeType::AtomFn,
        Rule::atom_fn_name => AstNodeType::AtomFnName,
        Rule::atom_fn_param => AstNodeType::AtomFnParam,
        Rule::atom_fn_params => AstNodeType::AtomFnParams,

        Rule::const_statement => AstNodeType::StatementConst,
        Rule::const_statement_name => AstNodeType::StatementConstName,

        Rule::fn_statement => AstNodeType::StatementFn,
        Rule::fn_statement_name => AstNodeType::StatementFnName,
        Rule::fn_statement_body => AstNodeType::StatementFnBody,

        Rule::emit_statement => AstNodeType::StatementEmit,

        Rule::EOI => AstNodeType::IGNORED,
        _ => AstNodeType::Unknown {
            rule_name: format!("{:?}", p.as_rule()),
        },
    };

    let option = node_type.must_capture_value()
        .then(|| p.as_str().to_string());

    let children: Result<Vec<AstNode>, _> = p.into_inner()
        .map(parse_ast_pair)
        .collect();

    Ok(
        AstNode {
            node_type: node_type,
            value: option,
            children: children?,
        }
    )
}
