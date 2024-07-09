use crate::compiler::ast::AstNodeType;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) enum Error {
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

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnexpectedNode { expected, actual } => {
                write!(
                    f,
                    "Unexpected node. Expected {:?}, got {:?}",
                    expected, actual
                )
            }
            Error::MalformedNodeValue { message } => {
                write!(f, "Malformed node value: {}", message)
            }
            Error::MissingContent { node_type } => {
                write!(f, "Missing content for node type {:?}", node_type)
            }
            Error::UnexpectedChildren {
                node_type,
                children,
            } => {
                write!(
                    f,
                    "Unexpected children for node type {:?}: {:?}",
                    node_type, children
                )
            }
            Error::DuplicateNode => {
                write!(f, "Duplicate node")
            }
        }
    }
}

impl std::error::Error for Error {}
