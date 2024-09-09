use std::env;
use std::fs;
use std::path::Path;

mod annotation;
mod line;
mod output_adapter;

use annotation::Annotation;
use line::{CommentLine, FileType};
use output_adapter::{OutputAdapter, JsonAdapter, YamlAdapter};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path> [--yaml|--json]", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let output_format = if args.len() > 2 && args[2] == "--yaml" {
        OutputAdapter::Yaml(YamlAdapter)
    } else {
        OutputAdapter::Json(JsonAdapter)
    };

    let file_type = FileType::from(file_path.to_path_buf());
    let content = fs::read_to_string(file_path).expect("Failed to read file");

    let annotations = extract_annotations(&content, &file_type);
    let output = output_format.format(&annotations);
    println!("{}", output);
}

fn extract_annotations(content: &str, file_type: &FileType) -> Vec<Annotation> {
    let lines: Vec<CommentLine> = content
        .lines()
        .enumerate()
        .map(|(i, line)| CommentLine::new(line.to_string(), i + 1))
        .collect();

    let mut annotations = Vec::new();

    for line in lines {
        if line.contains(&format!("{}@", file_type.comment_prefix())) {
            if let Some(annotation) = parse_annotation(&line) {
                annotations.push(annotation);
            }
        }
    }

    annotations
}

fn parse_annotation(line: &CommentLine) -> Option<Annotation> {
    let text = line.text();
    let at_pos = text.find('@')?;
    let colon_pos = text[at_pos..].find(':')?;
    
    let kind = text[at_pos + 1..at_pos + colon_pos].trim().to_string();
    let content = text[at_pos + colon_pos + 1..].trim().to_string();

    Some(Annotation { kind, content })
}
