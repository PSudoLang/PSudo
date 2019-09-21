use crate::coretypes::{CompileSession, RichDebug, ControlFlowExpression};
use crate::util::indented;

impl RichDebug for ControlFlowExpression {
    fn rich_debug(&self, session: &CompileSession) -> String {
        match self {
            ControlFlowExpression::If(r#if) => r#if.rich_debug(session),
            ControlFlowExpression::Return(_, expr) => {
                format!("Return {{\n{}\n}}", indented(expr.rich_debug(session)))
            }
            _ => "Unknown Control Flow Expression".into(),
        }
    }
}