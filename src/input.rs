use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;

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
    /// Get [Tree Sitter query](tree_sitter::Query) for file type.
    pub fn tree_sitter_query(&self) -> &'static tree_sitter::Query {
        match self {
            FileType::Python => &TS_QUERY_PYTHON,
            FileType::Rust => &TS_QUERY_RUST,
            FileType::JavaScript => &TS_QUERY_JAVASCRIPT,
        }
    }

    /// Get [Tree Sitter language](tree_sitter::Language) object from file type.
    pub fn tree_sitter_language(&self) -> tree_sitter::Language {
        match self {
            FileType::Python => tree_sitter_python::LANGUAGE.into(),
            FileType::Rust => tree_sitter_rust::LANGUAGE.into(),
            FileType::JavaScript => tree_sitter_javascript::LANGUAGE.into(),
        }
    }
}

/// Read content of file in `path` to a String.
///
/// # Errors
/// If there's a problem reading the file (e.g. if it doesn't exist).
pub fn read_file(path: &PathBuf) -> Result<String> {
    fs::read_to_string(path).map_err(|e| anyhow::anyhow!("Failed to read file: {}", e))
}

/// Determine file type from `path` extension. See [FileType].
///
/// # Errors
/// If the file type is not supported.
pub fn determine_file_type(path: &PathBuf) -> Result<FileType> {
    FileType::try_from(path)
}

/// Find all files from support file types. See [FileType].
///
/// Ignores files in `.gitignore` and `.ignore`, follows symlinks and ignores hidden files.
///
/// # Arguments
///
/// * `path` - Path to directory to search recursively.
///
/// # Returns
/// Paths of files from supported file types.
///
/// # Errors
/// If there are errors navigating the file system.
pub fn scan_directory(path: &PathBuf) -> Result<Vec<PathBuf>> {
    Ok(ignore::WalkBuilder::new(path)
        .follow_links(true)
        .hidden(false)
        .build()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_some_and(|ft| ft.is_file()))
        .map(|e| e.into_path())
        .filter(|p| determine_file_type(p).is_ok())
        .collect())
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
