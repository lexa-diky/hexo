use std::fmt::Display;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

#[derive(Debug)]
pub(crate) struct AstNode {
    pub(crate) node_type: AstNodeType,
    pub(crate) value: Option<String>,
    pub(crate) children: Vec<AstNode>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum AstNodeType {
    // Container
    File,

    StatementConst,
    StatementConstName,
    StatementConstParams,

    StatementEmit,

    AtomUtf8,
    AtomHex,
    AtomConst,
    AtomFn,

    AtomFnName,

    // Fallback
    IGNORED,
    Unknown {
        rule_name: String
    },
}

impl AstNodeType {
    fn must_capture_value(&self) -> bool {
        match self {
            AstNodeType::AtomUtf8
            | AstNodeType::AtomHex
            | AstNodeType::AtomFnName
            | AstNodeType::StatementConstName
            | AstNodeType::AtomConst => true,
            _ => false
        }
    }
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub(crate) struct HexoParser;

pub(crate) fn parse_ast(file_name: String, pairs: Pairs<Rule>) -> AstNode {
    return AstNode {
        node_type: AstNodeType::File,
        value: Some(file_name),
        children: pairs.map(parse_ast_pair).collect(),
    };
}

fn parse_ast_pair(p: Pair<Rule>) -> AstNode {
    let node_type = match p.as_rule() {
        Rule::atom_utf8 => AstNodeType::AtomUtf8,
        Rule::atom_hex => AstNodeType::AtomHex,
        Rule::atom_const => AstNodeType::AtomConst,
        Rule::atom_fn => AstNodeType::AtomFn,
        Rule::atom_fn_name => AstNodeType::AtomFnName,

        Rule::const_statement => AstNodeType::StatementConst,
        Rule::const_statement_name => AstNodeType::StatementConstName,
        Rule::atom_fn_params => AstNodeType::StatementConstParams,

        Rule::emit_statement => AstNodeType::StatementEmit,

        Rule::EOI => AstNodeType::IGNORED,
        _ => { AstNodeType::Unknown { rule_name: format!("{:?}", p.as_rule()) } }
    };
    let option = if node_type.must_capture_value() { Some(p.as_str().to_string()) } else { None };

    AstNode {
        node_type: node_type,
        value: option,
        children: p.into_inner().map(parse_ast_pair).collect(),
    }
}


impl Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // print as tree
        fn fmt_node(node: &AstNode, f: &mut std::fmt::Formatter, level: usize) -> std::fmt::Result {
            let indent = "  ".repeat(level);
            write!(f, "{}{:?} => {:?}\n", indent, node.node_type, node.value)?;
            for child in &node.children {
                fmt_node(child, f, level + 1)?;
            }
            Ok(())
        }

        fmt_node(self, f, 0)
    }
}
