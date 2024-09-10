pub mod annotation;
pub mod line;

use std::path::Path;

use line::CommentLine;

fn find_comment(file: &Path, line_no: usize) -> CommentLine {
    CommentLine::new("foo".to_string(), line_no)
}

fn extract_annotation(comment: CommentLine) {}
