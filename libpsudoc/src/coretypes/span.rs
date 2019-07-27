use crate::coretypes::LineColumn;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Span {
    // source_file: SourceFile,
    pub start: LineColumn,
    pub end: LineColumn,
}

impl Span {
    pub const FIRST_COLUMN: Span = Span {
        start: LineColumn::FIRST_COLUMN,
        end: LineColumn::FIRST_COLUMN,
    };

    pub fn joined(&self, other: Span) -> Option<Span> {
        // if self.source_file != other.source_file { return None; }

        let start = if self.start < other.start {
            self.start.clone()
        } else {
            other.start.clone()
        };
        let end = if self.end > other.end {
            self.end.clone()
        } else {
            other.end.clone()
        };

        Some(Span { start, end })
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
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
        if self.len() <= 0 {
            None
        } else {
            self[0].span().joined(self[self.len() - 1].span())
        }
        .unwrap_or(Span::FIRST_COLUMN)
    }
}
