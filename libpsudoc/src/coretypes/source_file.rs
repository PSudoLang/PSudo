use super::{CodeCharacter, CodeCharacterCategory};
use std::cell::RefCell;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

thread_local! {
    pub static ID: RefCell<u64> = RefCell::new(0);
}

#[derive(PartialEq, Debug)]
pub enum SourceFilePath {
    Real(PathBuf),
    Virtual(String),
}

#[derive(Debug)]
pub struct SourceFile {
    path: SourceFilePath,
    pub src: String,
    pub line_begins: Vec<usize>,
    pub unique_key: u64,
}

impl SourceFile {
    pub fn create_real<P: AsRef<Path>>(path: P) -> std::io::Result<SourceFile> {
        let mut f = File::open(&path)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;

        Ok(SourceFile::new(
            SourceFilePath::Real(path.as_ref().to_path_buf()),
            s,
        ))
    }

    pub fn create_virtual(path: String, src: String) -> SourceFile {
        SourceFile::new(SourceFilePath::Virtual(path), src)
    }

    fn new(path: SourceFilePath, src: String) -> SourceFile {
        SourceFile {
            path,
            src: src.clone(),
            line_begins: [vec![0], src
                .char_indices()
                .filter(|(_, character)| {
                    CodeCharacter::new(*character).category
                        == CodeCharacterCategory::VerticalSpace
                })
                .map(|(offset, _)| offset + 1)
                .collect()].concat(),
            unique_key: ID.with(|id| {
                *id.borrow_mut() += 1;
                *id.borrow()
            }),
        }
    }

    pub fn file_name(&self) -> String {
        match &self.path {
            SourceFilePath::Real(path) => {
                path.file_name().map_or("Unnammed".to_string(), |os_str| {
                    os_str.to_string_lossy().to_string()
                })
            }
            SourceFilePath::Virtual(s) => s.clone(),
        }
    }

    pub fn is_real(&self) -> bool {
        if let SourceFilePath::Real(_) = self.path {
            true
        } else {
            false
        }
    }
}

impl PartialEq for SourceFile {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Display for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.path {
            SourceFilePath::Real(path) => write!(f, "SourceFile({})", path.to_string_lossy()),
            SourceFilePath::Virtual(path) => write!(f, "SourceFile(virtual/{})", path),
        }
    }
}
