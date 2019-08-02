use crate::coretypes::{CompileSession, Span};

#[derive(Debug, Clone)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
    Help,
}

#[derive(Clone)]
pub struct Diagnostic {
    pub span: Span,
    pub level: DiagnosticLevel,
    pub message: String,
}

impl Diagnostic {
    pub fn emit_to(&self, session: &mut CompileSession) {
        session.add_diagnostic(self.clone());
    }
}
