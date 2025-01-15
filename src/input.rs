use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;
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

static TS_QUERY_PYTHON: LazyLock<tree_sitter::Query> = LazyLock::new(|| {
    tree_sitter::Query::new(&tree_sitter_python::LANGUAGE.into(), "(comment) @comment")
        .expect("Query must be valid")
});

static TS_QUERY_RUST: LazyLock<tree_sitter::Query> = LazyLock::new(|| {
    tree_sitter::Query::new(
        &tree_sitter_rust::LANGUAGE.into(),
        "(line_comment) @comment
(block_comment) @comment",
    )
    .expect("Query must be valid")
});

static TS_QUERY_JAVASCRIPT: LazyLock<tree_sitter::Query> = LazyLock::new(|| {
    tree_sitter::Query::new(
        &tree_sitter_javascript::LANGUAGE.into(),
        "(comment) @comment",
    )
    .expect("Query must be valid")
});

impl FileType {
    pub fn tree_sitter_query(&self) -> &'static tree_sitter::Query {
        match self {
            FileType::Python => &TS_QUERY_PYTHON,
            FileType::Rust => &TS_QUERY_RUST,
            FileType::JavaScript => &TS_QUERY_JAVASCRIPT,
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
}
