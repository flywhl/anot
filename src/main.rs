use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    anot::cli::run(args)?;
    Ok(())
}
