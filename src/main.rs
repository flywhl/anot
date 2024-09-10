mod annotation;
mod input;
mod parser;
mod output_adapter;
mod cli;
mod error;

use anyhow::Result;

fn main() -> Result<()> {
    let args = cli::parse_args();
    let content = input::read_file(&args.file)?;
    let file_type = input::determine_file_type(&args.file);
    let annotations = parser::extract_annotations(&content, &file_type)?;
    let output_format = match args.format {
        cli::OutputFormat::Json => output_adapter::OutputAdapter::Json(output_adapter::JsonAdapter),
        cli::OutputFormat::Yaml => output_adapter::OutputAdapter::Yaml(output_adapter::YamlAdapter),
    };
    let output = output_format.format(&annotations);
    println!("{}", output);
    Ok(())
}
