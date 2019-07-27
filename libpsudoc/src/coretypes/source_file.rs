use std::fmt::Display;
use std::path::PathBuf;

pub enum SourceFile {
    Real(PathBuf),
    Virtual(String),
}

impl SourceFile {
    pub fn is_real(&self) -> bool {
        if let SourceFile::Real(_) = self {
            true
        } else {
            false
        }
    }
}

impl Display for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceFile::Real(path) => write!(f, "SourceFile({})", path.to_string_lossy()),
            SourceFile::Virtual(path) => write!(f, "SourceFile(virtual/{})", path),
        }
    }
}
