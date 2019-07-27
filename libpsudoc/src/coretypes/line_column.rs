use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct LineColumn {
    pub offset: usize,
}

impl LineColumn {
    pub const FIRST_COLUMN: LineColumn = LineColumn { offset: 0 };

    /// 1-indexed line in the source file.
    pub fn line(&self) -> usize {
        // TODO
        1
    }
    /// 0-indexed column in the source file.
    pub fn column(&self) -> usize {
        // TODO
        0
    }
}

impl PartialEq for LineColumn {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset
    }
}

impl PartialOrd for LineColumn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.offset.cmp(&other.offset))
    }
}

impl Display for LineColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "L{}:{}", self.line(), self.column())
    }
}
