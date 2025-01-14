use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Location {
    pub file: PathBuf,
    pub line: usize,
    pub inline: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SyntaxContext {
    pub node_type: String,
    pub parent_type: String,
    pub associated_name: Option<String>,
    pub variable_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Annotation {
    pub kind: String,
    pub content: String,
    pub location: Location,
    pub context: SyntaxContext,
}
