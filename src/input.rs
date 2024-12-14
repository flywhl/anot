use anyhow::Result;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    Python,
    Rust,
    JavaScript,
    Unknown,
}

impl From<&PathBuf> for FileType {
    fn from(path: &PathBuf) -> Self {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("py") => FileType::Python,
            Some("rs") => FileType::Rust,
            Some("js") => FileType::JavaScript,
            _ => FileType::Unknown,
        }
    }
}

impl FileType {
    pub fn comment_prefix(&self) -> &'static str {
        match self {
            FileType::Python => "#",
            FileType::Rust => "//",
            FileType::JavaScript => "//",
            FileType::Unknown => "//",
        }
    }
}

pub fn read_file(path: &PathBuf) -> Result<String> {
    fs::read_to_string(path).map_err(|e| anyhow::anyhow!("Failed to read file: {}", e))
}

pub fn determine_file_type(path: &PathBuf) -> FileType {
    FileType::from(path)
}
