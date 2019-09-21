use crate::coretypes::{CompileSession, RichDebug, MemberExpression};
use crate::util::indented;

impl RichDebug for MemberExpression {
    fn rich_debug(&self, session: &CompileSession) -> String {
        match self {
            MemberExpression::Field(_, expression, field_name, is_null_safe) => format!(
                "Field{} \"{}\" {{\n{}\n}}",
                if *is_null_safe {
                    " with null-safety"
                } else {
                    ""
                },
                field_name,
                indented(expression.rich_debug(session))
            ),
            _ => "Unknown Member Expression".into(),
        }
    }
}