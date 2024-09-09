use std::fs;
use std::path::PathBuf;

use clap::Parser;
use serde::{Serialize, Deserialize};

mod annotation;
mod line;
mod output_adapter;

use annotation::Annotation;
use line::{CommentLine, FileType, TAG};
use output_adapter::{JsonAdapter, OutputAdapter, YamlAdapter};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the file to analyze
    #[arg(value_name = "FILE")]
    file: PathBuf,

    /// Output format
    #[arg(short, long, value_enum, default_value = "json")]
    format: OutputFormat,
}

#[derive(clap::ValueEnum, Clone)]
enum OutputFormat {
    Json,
    Yaml,
}

fn main() {
    let cli = Cli::parse();

    let file_type = FileType::from(cli.file.clone());
    let content = fs::read_to_string(&cli.file).expect("Failed to read file");

    let annotations = extract_annotations(&content, &file_type);
    let output_format = match cli.format {
        OutputFormat::Json => OutputAdapter::Json(JsonAdapter),
        OutputFormat::Yaml => OutputAdapter::Yaml(YamlAdapter),
    };
    let output = output_format.format(&annotations);
    println!("{}", output);
}

fn extract_annotations(content: &str, file_type: &FileType) -> Vec<Annotation> {
    let comment_prefix = file_type.comment_prefix();
    let anot_prefix = format!("{} {}", comment_prefix, TAG);
    content
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            if line.starts_with(&anot_prefix) {
                let comment_line = CommentLine::new(line.to_string(), i + 1);
                parse_annotation(&comment_line)
            } else {
                None
            }
        })
        .collect()
}

fn parse_annotation(line: &CommentLine) -> Option<Annotation> {
    let text = line.text();
    let at_pos = text.find('@')?;
    let colon_pos = text[at_pos..].find(':')?;

    let kind = text[at_pos + 1..at_pos + colon_pos].trim().to_string();
    let content = text[at_pos + colon_pos + 1..].trim().to_string();

    Some(Annotation { kind, content })
}
