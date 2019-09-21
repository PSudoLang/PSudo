use crate::coretypes::{CompileSession, RichDebug, Literal};

impl RichDebug for Literal {
    fn rich_debug(&self, session: &CompileSession) -> String {
        match self {
            Literal::Boolean(.., val) => format!("{:?}", val),
            Literal::String(.., val) => format!("{:?}", val),
            Literal::Integer(span) | Literal::Decimal(span) => span.source_text(session),
        }
    }
}