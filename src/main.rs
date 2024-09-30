mod corex;
use std::path::Path;
mod exec;

use clap::Parser;
use miette::Context;
use miette::Result;
use sxm::args;
use sxm::parser::parse_pipeline;
use sxm::setup_logging;

fn main() -> Result<()> {
    let cli = args::Cli::parse();

    setup_logging()?;

    let pipeline_file_path = cli.pipeline.clone();

    let path = Path::new(&pipeline_file_path);

    let pipeline = parse_pipeline(path).context("Pipeline parsing failed")?;

    exec::run_app(cli, pipeline)?;

    Ok(())
}
