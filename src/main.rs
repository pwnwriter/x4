mod corex;
mod exec;

use clap::Parser;
use miette::{Context, Result};
use sxm::args;
use sxm::helpers::setup_logging;
use sxm::parser::parse_pipeline;

fn main() -> Result<()> {
    let cli = args::Cli::parse();

    setup_logging()?;

    match cli.pipeline {
        Some(ref pipeline_file_path) => {
            let pipeline = parse_pipeline(pipeline_file_path).context("Pipeline parsing failed")?;

            exec::run_app(cli.clone(), pipeline)?;
        }
        None => {
            if let Some(ref file_path) = cli.file.from {
                println!("File upload path: {}", file_path);
            } else {
                println!("No file upload path provided.");
            }

            if let Some(ref remote_path) = cli.file.to {
                println!("Remote file path: {}", remote_path);
            } else {
                println!("No remote file path provided.");
            }
        }
    }

    Ok(())
}
