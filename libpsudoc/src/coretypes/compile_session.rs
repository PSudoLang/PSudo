use super::{Diagnostic, SourceFile};
use std::collections::HashMap;

pub struct CompileSession {
    diagnostics: Vec<Diagnostic>,
    files: HashMap<u64, SourceFile>,
}

impl CompileSession {
    pub fn new() -> CompileSession {
        CompileSession {
            diagnostics: Vec::new(),
            files: HashMap::new(),
        }
    }

    pub fn add_source_file(&mut self, file: SourceFile) {
        self.files.insert(file.unique_key, file);
    }

    pub fn get_source_file(&self, key: u64) -> Option<&SourceFile> {
        self.files.get(&key)
    }
}
