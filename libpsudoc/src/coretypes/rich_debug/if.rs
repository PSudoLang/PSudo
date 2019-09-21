use crate::coretypes::{CompileSession, RichDebug, If};
use crate::util::indented;

impl RichDebug for If {
    fn rich_debug(&self, session: &CompileSession) -> String {
        format!(
            "if {} {{\n{}\n}}{}",
            self.condition.rich_debug(session),
            indented(if self.if_branch.1.is_empty() {
                "empty block".into()
            } else {
                self.if_branch
                    .1
                    .iter()
                    .map(|statement| statement.rich_debug(session))
                    .collect::<Vec<_>>()
                    .join("\n")
            }),
            if let Some(else_branch) = &self.else_branch {
                format!(" {}", else_branch.rich_debug(session))
            } else {
                "".into()
            }
        )
    }
}