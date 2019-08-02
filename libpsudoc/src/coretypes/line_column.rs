use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum LineColumn {
    Invalid,
    Valid { line: usize, column: usize },
}

impl LineColumn {
    pub fn line(&self) -> usize {
        match self {
            LineColumn::Invalid => 0,
            LineColumn::Valid { line, .. } => *line,
        }
    }

    pub fn column(&self) -> usize {
        match self {
            LineColumn::Invalid => 0,
            LineColumn::Valid { column, .. } => *column,
        }
    }
}

impl PartialOrd for LineColumn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (LineColumn::Invalid, LineColumn::Invalid) => return Some(Ordering::Equal),
            (LineColumn::Invalid, _) => return Some(Ordering::Less),
            (_, LineColumn::Invalid) => return Some(Ordering::Greater),
            (_, _) => {}
        }

        if self.line() != other.line() {
            Some(self.line().cmp(&other.line()))
        } else {
            Some(self.column().cmp(&other.column()))
        }
    }
}

impl Display for LineColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineColumn::Invalid => write!(f, "Invalid Offset"),
            LineColumn::Valid { line, column } => write!(f, "L{}:{}", line, column),
        }
    }
}
