use super::SourceFile;
use std::collections::HashMap;

pub struct SourceMap {
    sources: HashMap<String, SourceFile>,
}
