use crate::coretypes::{CompileSession, RichDebug, Expression};
use crate::util::indented;

impl RichDebug for Expression {
    fn rich_debug(&self, session: &CompileSession) -> String {
        match self {
            Expression::Unit(..) => "()".into(),
            Expression::Group(_, expression) => expression.rich_debug(session),
            Expression::Tuple(_, expressions) => format!(
                "Tuple {{\n{}\n}}",
                indented(
                    expressions
                        .iter()
                        .map(|expression| expression.rich_debug(session))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            ),
            Expression::Literal(literal) => literal.rich_debug(session),
            Expression::Array(_, expressions) => format!(
                "Array {{\n{}\n}}",
                indented(
                    expressions
                        .iter()
                        .map(|expression| expression.rich_debug(session))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            ),
            Expression::Operator(operator_expression) => operator_expression.rich_debug(session),
            Expression::ControlFlow(control_flow_expression) => {
                control_flow_expression.rich_debug(session)
            }
            _ => "Unknown Expression".into(),
        }
    }
}