use anyhow::Result;
use clap::Parser;
use std::{collections::HashSet, path::PathBuf};

use crate::{input, parser, render};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the file to analyze
    #[arg(value_name = "PATH")]
    pub path: PathBuf,

    /// Output format
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: OutputFormat,

    /// Comma-separated list of tags to show
    #[arg(long, value_delimiter = ',')]
    pub tags: Vec<String>,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        let mut annotations = Vec::new();

        if self.path.is_dir() {
            let files = input::scan_directory(&self.path)?;
            for file in files {
                let content = input::read_file(&file)?;
                let file_type = input::determine_file_type(&file)?;
                annotations.extend(parser::extract_annotations(&content, &file_type, &file)?);
            }
        } else {
            let content = input::read_file(&self.path)?;
            let file_type = input::determine_file_type(&self.path)?;
            annotations = parser::extract_annotations(&content, &file_type, &self.path)?;
        }

        let tags: HashSet<String> = self.tags.iter().cloned().collect();
        let filtered_annotations = if !self.tags.is_empty() {
            annotations
                .into_iter()
                .filter(|ann| tags.contains(&ann.kind))
                .collect()
        } else {
            annotations
        };

        let output_format = match self.format {
            OutputFormat::Json => render::RenderAdapter::Json(render::JsonAdapter),
            OutputFormat::Yaml => render::RenderAdapter::Yaml(render::YamlAdapter),
        };
        let output = output_format.format(&filtered_annotations)?;
        println!("{}", output);
        Ok(())
    }
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Json,
    Yaml,
}

pub fn run(args: Vec<String>) -> Result<(), anyhow::Error> {
    Cli::parse_from(args).run()
}
