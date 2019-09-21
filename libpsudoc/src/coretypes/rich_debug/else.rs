use crate::coretypes::{CompileSession, RichDebug, Else};
use crate::util::indented;

impl RichDebug for Else {
    fn rich_debug(&self, session: &CompileSession) -> String {
        match self {
            Else::ElseIf(_, r#if) => format!("else {}", r#if.rich_debug(session)),
            Else::Else(_, block) => format!(
                "else {{\n{}\n}}",
                indented(if block.1.is_empty() {
                    "empty block".into()
                } else {
                    block
                        .1
                        .iter()
                        .map(|statement| statement.rich_debug(session))
                        .collect::<Vec<_>>()
                        .join("\n")
                })
            ),
        }
    }
}
