use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    Python,
    Rust,
    JavaScript,
}

impl TryFrom<&PathBuf> for FileType {
    type Error = anyhow::Error;
    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("py") => Ok(FileType::Python),
            Some("rs") => Ok(FileType::Rust),
            Some("js") => Ok(FileType::JavaScript),
            _ => Err(anyhow::anyhow!("Invalid file extension: {:?}.", path)),
        }
    }
}

impl FileType {
    pub fn tree_sitter_query(&self) -> &'static str {
        match self {
            FileType::Python => "(comment) @comment",
            FileType::Rust => {
                "(line_comment) @comment
                (block_comment) @comment"
            }
            FileType::JavaScript => "(comment) @comment",
        }
    }

    pub fn tree_sitter_language(&self) -> tree_sitter::Language {
        match self {
            FileType::Python => tree_sitter_python::LANGUAGE.into(),
            FileType::Rust => tree_sitter_rust::LANGUAGE.into(),
            FileType::JavaScript => tree_sitter_javascript::LANGUAGE.into(),
        }
    }
}

pub fn read_file(path: &PathBuf) -> Result<String> {
    fs::read_to_string(path).map_err(|e| anyhow::anyhow!("Failed to read file: {}", e))
}

pub fn determine_file_type(path: &PathBuf) -> Result<FileType> {
    FileType::try_from(path)
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
            if determine_file_type(&path).is_ok() {
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
        assert_eq!(
            determine_file_type(&PathBuf::from("test.py")).unwrap(),
            FileType::Python
        );
        assert_eq!(
            determine_file_type(&PathBuf::from("test.rs")).unwrap(),
            FileType::Rust
        );
        assert_eq!(
            determine_file_type(&PathBuf::from("test.js")).unwrap(),
            FileType::JavaScript
        );
        assert!(determine_file_type(&PathBuf::from("test.txt")).is_err());
    }

    #[test]
    fn test_queries_are_valid() {
        for file_type in [FileType::Python, FileType::Rust, FileType::JavaScript] {
            let language = file_type.tree_sitter_language();
            let query = file_type.tree_sitter_query();
            assert!(
                tree_sitter::Query::new(&language, query).is_ok(),
                "Testing query from {:?}.",
                file_type
            );
        }
    }
}
