use miette::{Context, Result};

use clap::Parser;
use sxm::engine::args;
use sxm::engine::configuration::parse_pipeline;
use sxm::helpers::{run_app, setup_logging};

fn main() -> Result<()> {
    let cli = args::Cli::parse();

    setup_logging()?;

    match cli.pipeline {
        Some(ref pipeline_file_path) => {
            let pipeline = parse_pipeline(pipeline_file_path).context("Pipeline parsing failed")?;

            run_app(cli.clone(), pipeline)?;
        }
        None => {
            println!("This is something");
        }
    }

    Ok(())
}
