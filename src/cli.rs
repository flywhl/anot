use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the file to analyze
    #[arg(value_name = "FILE")]
    pub file: PathBuf,

    /// Output format
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: OutputFormat,
}

#[derive(clap::ValueEnum, Clone)]
pub enum OutputFormat {
    Json,
    Yaml,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
