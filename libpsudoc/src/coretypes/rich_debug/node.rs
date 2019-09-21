use crate::coretypes::{RichDebug, Node, CompileSession};

impl RichDebug for Node {
    fn rich_debug(&self, session: &CompileSession) -> String {
        match self {
            Node::Root(nodes) => format!(
                "Root {{\n{}\n}}",
                nodes
                    .iter()
                    .flat_map(|node| node
                        .rich_debug(session)
                        .split('\n')
                        .map(|line| String::from("  ") + line)
                        .collect::<Vec<_>>())
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
            Node::Comment(span, is_block_comment) => format!(
                "{}Comment {:?}",
                if *is_block_comment { "Block" } else { "Line" },
                span.source_text(session)
            ),
            Node::Statement(statement) => statement.rich_debug(session),
            _ => "Unknown Node".into(),
        }
    }
}
