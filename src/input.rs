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
    pub fn tree_sitter_query(&self) -> Option<&'static str> {
        match self {
            FileType::Python => Some("(comment) @comment"),
            FileType::Rust => Some(
                "(line_comment) @comment
                (block_comment) @comment",
            ),
            FileType::JavaScript => Some("(comment) @comment"),
            FileType::Unknown => None,
        }
    }

    pub fn tree_sitter_language(&self) -> Option<tree_sitter::Language> {
        match self {
            FileType::Python => Some(tree_sitter_python::LANGUAGE.into()),
            FileType::Rust => Some(tree_sitter_rust::LANGUAGE.into()),
            FileType::JavaScript => Some(tree_sitter_javascript::LANGUAGE.into()),
            FileType::Unknown => None,
        }
    }
}

use walkdir::WalkDir;

pub fn read_file(path: &PathBuf) -> Result<String> {
    fs::read_to_string(path).map_err(|e| anyhow::anyhow!("Failed to read file: {}", e))
}

pub fn determine_file_type(path: &PathBuf) -> FileType {
    FileType::from(path)
}

pub fn scan_directory(path: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let path = entry.path().to_path_buf();
            if determine_file_type(&path) != FileType::Unknown {
                files.push(path);
            }
        }
    }
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_type_detection() {
        assert_eq!(FileType::from(&PathBuf::from("test.py")), FileType::Python);
        assert_eq!(FileType::from(&PathBuf::from("test.rs")), FileType::Rust);
        assert_eq!(
            FileType::from(&PathBuf::from("test.js")),
            FileType::JavaScript
        );
        assert_eq!(
            FileType::from(&PathBuf::from("test.txt")),
            FileType::Unknown
        );
    }

    #[test]
    fn test_queries_are_valid() {
        for file_type in [FileType::Python, FileType::Rust, FileType::JavaScript] {
            if let (Some(language), Some(query)) = (
                file_type.tree_sitter_language(),
                file_type.tree_sitter_query(),
            ) {
                assert!(
                    tree_sitter::Query::new(&language, query).is_ok(),
                    "Testing query from {:?}.",
                    file_type
                );
            }
        }
    }
}
