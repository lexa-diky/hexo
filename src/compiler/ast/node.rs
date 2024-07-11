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
        matches!(
            self,
            AstNodeType::AtomUtf8
                | AstNodeType::AtomHex
                | AstNodeType::AtomFnName
                | AstNodeType::StatementConstName
                | AstNodeType::AtomBaseNumberBase
                | AstNodeType::AtomBaseNumberValue
                | AstNodeType::StatementFnName
                | AstNodeType::AtomFnParamIdentifier
                | AstNodeType::AtomConst
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct AstNode {
    node_type: AstNodeType,
    content: Option<String>,
    children: Vec<AstNode>,
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

    pub(crate) fn children(&self) -> &Vec<AstNode> {
        &self.children
    }

    pub(crate) fn node_type(&self) -> AstNodeType {
        self.node_type
    }

    pub(crate) fn content(&self) -> Option<&String> {
        self.content.as_ref()
    }
}
