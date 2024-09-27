mod corex;
mod exec;

use clap::Parser;
use miette::{Context, Result};
use std::path::Path;
use sxm::args;
use sxm::parser::parse_pipeline;

fn main() -> Result<()> {
    let cli = args::Cli::parse();

    let pipeline_file_path = cli.pipeline.clone();

    let path = Path::new(&pipeline_file_path);

    let pipeline = parse_pipeline(path).context("Pipeline parsing failed")?;

    exec::run_app(pipeline, cli)?;

    Ok(())
}
