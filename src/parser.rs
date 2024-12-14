use crate::annotation::Annotation;
use crate::input::FileType;
use anyhow::Result;

pub const TAG: &str = "@";

pub fn extract_annotations(content: &str, file_type: &FileType) -> Result<Vec<Annotation>> {
    let comment_prefix = file_type.comment_prefix();
    let anot_prefix = format!("{} {}", comment_prefix, TAG);
    Ok(content
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            if line.contains(&anot_prefix) {
                parse_annotation(line, i + 1)
            } else if line.contains(&format!("{}{}", comment_prefix, TAG)) {
                // Handle inline comments
                let parts: Vec<&str> = line.split(comment_prefix).collect();
                if let Some(comment) = parts.last() {
                    if comment.trim().starts_with(TAG) {
                        return parse_annotation(comment.trim(), i + 1);
                    }
                }
                None
            } else {
                None
            }
        })
        .collect())
}

fn parse_annotation(line: &str, _line_number: usize) -> Option<Annotation> {
    let at_pos = line.find('@')?;
    let colon_pos = line[at_pos..].find(':')?;

    let kind = line[at_pos + 1..at_pos + colon_pos].trim().to_string();
    let content = line[at_pos + colon_pos + 1..].trim().to_string();

    Some(Annotation { kind, content })
}
