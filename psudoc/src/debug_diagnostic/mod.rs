mod prelude;
mod source_line;

pub(self) use crate::util::SliceByChar;
pub(self) use colored::{Color, ColoredString, Colorize};
pub(self) use libpsudoc::coretypes::{
    CompileSession, Diagnostic, DiagnosticLevel, LineColumn, SourceFile,
};
pub(self) use unicode_width::UnicodeWidthStr;

use prelude::*;
use source_line::*;

struct DiagnosticRenderInformation<'a> {
    line_number_length: usize,
    diagnostic: &'a Diagnostic,
    source_file: &'a SourceFile,
    compile_session: &'a CompileSession,
}

impl DiagnosticRenderInformation<'_> {
    pub fn start(&self) -> LineColumn {
        self.diagnostic.span.start(self.compile_session)
    }

    pub fn end(&self) -> LineColumn {
        self.diagnostic.span.end(self.compile_session)
    }

    pub fn diagnostic_color(&self) -> Color {
        match self.diagnostic.level {
            DiagnosticLevel::Error => Color::Red,
            DiagnosticLevel::Warning => Color::Yellow,
            DiagnosticLevel::Note => Color::Yellow,
            DiagnosticLevel::Help => Color::Blue,
        }
    }

    pub fn diagnostic_label(&self) -> &'static str {
        match self.diagnostic.level {
            DiagnosticLevel::Error => "error",
            DiagnosticLevel::Warning => "warning",
            DiagnosticLevel::Note => "note",
            DiagnosticLevel::Help => "help",
        }
    }
}

trait DiagnosticRenderer {
    fn render(info: &DiagnosticRenderInformation);
}

pub fn debug_diagnostic<'a>(
    diagnostic: &'a Diagnostic,
    source_file: &'a SourceFile,
    compile_session: &'a CompileSession,
) {
    let start = diagnostic.span.start(&compile_session);
    let end = diagnostic.span.end(&compile_session);
    let line_number_length =
        std::cmp::max(start.line().to_string().len(), end.line().to_string().len());

    let info = DiagnosticRenderInformation {
        line_number_length,
        diagnostic,
        source_file,
        compile_session,
    };

    DiagnosticPrelude::render(&info);
    SourceView::render(&info);
}
