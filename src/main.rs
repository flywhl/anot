mod annotation;
mod cli;
mod error;
mod input;
mod parser;
mod render;

use anyhow::Result;

fn main() -> Result<()> {
    let args = cli::parse_args();
    let content = input::read_file(&args.file)?;
    let file_type = input::determine_file_type(&args.file);
    let annotations = parser::extract_annotations(&content, &file_type)?;
    let output_format = match args.format {
        cli::OutputFormat::Json => render::RenderAdapter::Json(render::JsonAdapter),
        cli::OutputFormat::Yaml => render::RenderAdapter::Yaml(render::YamlAdapter),
    };
    let output = output_format.format(&annotations)?;
    println!("{}", output);
    Ok(())
}
