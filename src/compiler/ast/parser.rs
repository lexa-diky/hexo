use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::compiler::ast::{AstNode, AstNodeType};

#[derive(Debug)]
pub(crate) enum AstParserError {
    PestError(Error<Rule>),
    UnknownRule { rule_name: String },
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub(crate) struct AstPestParser;

pub(crate) struct AstParser {}

impl AstParser {
    pub(crate) fn new() -> Self {
        AstParser {}
    }

    pub(crate) fn parse(&self, source: String) -> Result<AstNode, AstParserError> {
        let pairs = AstPestParser::parse(Rule::file, source.as_str())
            .map_err(|error| AstParserError::PestError(error))?;

        let children: Result<Vec<AstNode>, _> = pairs.map(parse_ast_pair)
            .filter_map(filter_ignored_token)
            .collect();

        return Ok(AstNode::new(AstNodeType::File, None, children?));
    }
}

fn filter_ignored_token(result: Result<Option<AstNode>, AstParserError>) -> Option<Result<AstNode, AstParserError>> {
    match result {
        Ok(None) => None,
        Ok(Some(value)) => Some(Ok(value)),
        Err(error) => Some(Err(error))
    }
}

fn parse_ast_pair(p: Pair<Rule>) -> Result<Option<AstNode>, AstParserError> {
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

        Rule::EOI => return Ok(None),
        _ => return Err(AstParserError::UnknownRule {
            rule_name: format!("{:?}", p.as_rule()),
        }),
    };

    let node_value = node_type
        .must_capture_value()
        .then(|| p.as_str().to_string());

    let children: Result<Vec<AstNode>, _> = p.into_inner()
        .map(parse_ast_pair)
        .filter_map(filter_ignored_token)
        .collect();

    Ok(
        Some(
            AstNode::new(node_type, node_value, children?)
        )
    )
}