mod args;
mod parser;
use clap::Parser;
use miette::{Context, Result};
use parser::parse_pipeline;
use std::path::Path;

fn main() -> Result<()> {
    let cli = args::Cli::parse();
    let pipeline_file_path = cli.pipeline_file;
    let path = Path::new(&pipeline_file_path);
    let pipeline =
        parse_pipeline(path).context("Failed to parse the pipeline from the given JSON file")?;

    for server in pipeline.servers {
        println!("{:?}", server.name);
    }

    Ok(())
}
