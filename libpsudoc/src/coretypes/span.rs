use super::CompileSession;
use crate::coretypes::{Diagnostic, DiagnosticLevel, LineColumn, RichDebug, SourceFile};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Span {
    pub source_file: Option<u64>,
    pub start_offset: usize,
    pub end_offset: usize,
}

impl Span {
    pub const FIRST_COLUMN: Span = Span {
        source_file: None,
        start_offset: 0,
        end_offset: 0,
    };

    pub fn length(&self) -> usize {
        self.end_offset - self.start_offset
    }

    pub fn source_file<'a>(&self, session: &'a CompileSession) -> Option<&'a SourceFile> {
        self.source_file
            .and_then(|key| session.get_source_file(key))
    }

    pub fn start(&self, session: &CompileSession) -> LineColumn {
        self.source_file(session)
            .map(|source_file| {
                let (line, column) = match source_file.line_begins.binary_search(&self.start_offset)
                {
                    Ok(line) => (line + 1, 0),
                    Err(line) => (
                        line,
                        self.start_offset - source_file.line_begins.get(line - 1).unwrap_or(&0),
                    ),
                };

                LineColumn::Valid { line, column }
            })
            .unwrap_or(LineColumn::Invalid)
    }

    pub fn end(&self, session: &CompileSession) -> LineColumn {
        self.source_file(session)
            .map(|source_file| {
                let (line, column) = match source_file.line_begins.binary_search(&self.end_offset) {
                    Ok(line) => (line + 1, 0),
                    Err(line) => (
                        line,
                        self.end_offset - source_file.line_begins.get(line - 1).unwrap_or(&0),
                    ),
                };

                LineColumn::Valid { line, column }
            })
            .unwrap_or(LineColumn::Invalid)
    }

    pub fn joined(&self, other: Span) -> Option<Span> {
        if self.length() > 0 && other.length() > 0 && self.source_file != other.source_file {
            return None;
        }

        let start_offset = if self.start_offset < other.start_offset {
            self.start_offset
        } else {
            other.start_offset
        };
        let end_offset = if self.end_offset > other.end_offset {
            self.end_offset
        } else {
            other.end_offset
        };

        Some(Span {
            source_file: self.source_file,
            start_offset,
            end_offset,
        })
    }

    pub fn source_text(&self, session: &CompileSession) -> String {
        self.source_file(session)
            .map(|source_file| {
                source_file
                    .src
                    .chars()
                    .skip(self.start_offset)
                    .take(self.length())
                    .collect::<String>()
            })
            .unwrap_or_else(|| "".to_string())
    }

    pub fn diagnostic_error<T: ToString>(&self, message: T) -> Diagnostic {
        Diagnostic {
            span: self.clone(),
            level: DiagnosticLevel::Error,
            message: message.to_string(),
        }
    }

    pub fn diagnostic_warning<T: ToString>(&self, message: T) -> Diagnostic {
        Diagnostic {
            span: self.clone(),
            level: DiagnosticLevel::Warning,
            message: message.to_string(),
        }
    }

    pub fn diagnostic_note<T: ToString>(&self, message: T) -> Diagnostic {
        Diagnostic {
            span: self.clone(),
            level: DiagnosticLevel::Note,
            message: message.to_string(),
        }
    }

    pub fn diagnostic_help<T: ToString>(&self, message: T) -> Diagnostic {
        Diagnostic {
            span: self.clone(),
            level: DiagnosticLevel::Help,
            message: message.to_string(),
        }
    }
}

impl RichDebug for Span {
    fn rich_debug(&self, session: &CompileSession) -> String {
        format!("{}..{}", self.start(session), self.end(session))
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start_offset, self.end_offset)
    }
}

pub trait Spanned {
    fn span(&self) -> Span;
}

impl<T> Spanned for Vec<T>
where
    T: Spanned,
{
    fn span(&self) -> Span {
        if self.is_empty() {
            None
        } else {
            self[0].span().joined(self[self.len() - 1].span())
        }
        .unwrap_or(Span::FIRST_COLUMN)
    }
}
