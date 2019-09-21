use crate::coretypes::{CompileSession, RichDebug, Statement};
use crate::util::indented;

impl RichDebug for Statement {
    fn rich_debug(&self, session: &CompileSession) -> String {
        match self {
            Statement::Output(_, expressions, to_print_linefeed) => format!(
                "Output{} {{\n{}\n}}",
                if *to_print_linefeed {
                    ""
                } else {
                    "(No Linefeed)"
                },
                indented(
                    expressions
                        .iter()
                        .map(|expression| expression.rich_debug(session))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            ),
            Statement::Expression(expression) => {
                format!("Statement :: {}", expression.rich_debug(session))
            }
            Statement::Declaration(declaration) => declaration.rich_debug(session),
            _ => "Unknown Statement".into(),
        }
    }
}
