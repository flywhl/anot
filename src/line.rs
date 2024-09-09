use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    Python,
    Unknown,
}

impl From<PathBuf> for FileType {
    fn from(path: PathBuf) -> Self {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("py") => FileType::Python,
            _ => FileType::Unknown,
        }
    }
}

impl FileType {
    fn comment_prefix(&self) -> &'static str {
        match self {
            FileType::Python => "#",
            FileType::Unknown => "//", // Default to C-style comments
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommentLine {
    number: usize,
    text: String,
}

impl CommentLine {
    pub fn new(text: String, number: usize) -> Self {
        Self { text, number }
    }

    fn text(&self) -> &str {
        &self.text
    }

    fn number(&self) -> &usize {
        &self.number
    }

    fn set_text(&mut self, value: String) {
        self.text = value
    }

    fn contains(&self, substr: &str) -> bool {
        self.text.contains(substr)
    }
}
