use std::fmt::Display;
use std::path::PathBuf;

pub enum SourceFilePath {
    Real(PathBuf),
    Virtual(String),
}

impl SourceFilePath {
    pub fn file_name(&self) -> String {
        match self {
            SourceFilePath::Real(path) => {
                path.file_name().map_or("Unnammed".to_string(), |os_str| {
                    os_str.to_string_lossy().to_string()
                })
            }
            SourceFilePath::Virtual(s) => s.clone(),
        }
    }

    pub fn is_real(&self) -> bool {
        if let SourceFilePath::Real(_) = self {
            true
        } else {
            false
        }
    }
}

impl Display for SourceFilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceFilePath::Real(path) => write!(f, "SourceFilePath({})", path.to_string_lossy()),
            SourceFilePath::Virtual(path) => write!(f, "SourceFilePath(virtual/{})", path),
        }
    }
}

pub struct SourceFile {
    path: SourceFilePath,
    src: String,
    line_begins: Vec<usize>,
}
