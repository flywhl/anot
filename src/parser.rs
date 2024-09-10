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
            if line.starts_with(&anot_prefix) {
                parse_annotation(line, i + 1)
            } else {
                None
            }
        })
        .collect())
}

fn parse_annotation(line: &str, line_number: usize) -> Option<Annotation> {
    let at_pos = line.find('@')?;
    let colon_pos = line[at_pos..].find(':')?;

    let kind = line[at_pos + 1..at_pos + colon_pos].trim().to_string();
    let content = line[at_pos + colon_pos + 1..].trim().to_string();

    Some(Annotation { kind, content })
}
