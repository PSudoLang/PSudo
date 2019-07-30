use super::{Statement};
use crate::coretypes::{RichDebug, CompileSession, Span, Spanned};

type IsBlockComment = bool;

pub enum Node {
    Root(Vec<Node>),
    CompilerDirective { name: String },
    Comment(Span, IsBlockComment),
    Statement(Statement),
}

impl Spanned for Node {
    fn span(&self) -> Span {
        match self {
            Node::Root(vec) => vec.span(),
            // TODO
            Node::CompilerDirective { .. } => Span::FIRST_COLUMN,
            Node::Comment(span, ..) => span.clone(),
            Node::Statement(statement) => statement.span(),
        }
    }
}

pub struct Type {
    name: String,
}

impl RichDebug for Node {
    fn rich_debug(&self, session: &CompileSession) -> String {
        match self {
            Node::Root(nodes) => format!(
                "Root {{\n{}\n}}",
                nodes
                    .iter()
                    .map(|node| node.rich_debug(session))
                    .collect::<Vec<_>>()
                    .join("\n")
                    .split('\n')
                    .map(|line| format!("  {}", line))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
            Node::Comment(span, is_block_comment) => format!(
                "{}Comment {:?}",
                if *is_block_comment { "Block" } else { "Line" },
                span.source_text(session)
            ),
            _ => "Unknown Node".to_string(),
        }
    }
}
