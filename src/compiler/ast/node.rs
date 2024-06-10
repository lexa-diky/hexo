#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum AstNodeType {
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
    AtomFnParamValue,
    AtomFnParamIdentifier,
    AtomFnParams,
    AtomBaseNumber,
    AtomBaseNumberBase,
    AtomBaseNumberValue,

    AtomFnName,
}

impl AstNodeType {
    pub(crate) fn must_capture_value(&self) -> bool {
        match self {
            AstNodeType::AtomUtf8
            | AstNodeType::AtomHex
            | AstNodeType::AtomFnName
            | AstNodeType::StatementConstName
            | AstNodeType::AtomBaseNumberBase
            | AstNodeType::AtomBaseNumberValue
            | AstNodeType::StatementFnName
            | AstNodeType::AtomFnParamIdentifier
            | AstNodeType::AtomConst => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct AstNode {
    pub(crate) node_type: AstNodeType,
    pub(crate) content: Option<String>,
    pub(crate) children: Vec<AstNode>,
}

impl AstNode {
    pub(crate) fn new(
        node_type: AstNodeType,
        content: Option<String>,
        children: Vec<AstNode>,
    ) -> Self {
        AstNode {
            node_type,
            content,
            children,
        }
    }
}
