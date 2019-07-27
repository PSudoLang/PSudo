use crate::coretypes::Span;

pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
    Help,
}

pub struct Diagnostic {
    span: Span,
    level: DiagnosticLevel,
    message: String,
}
