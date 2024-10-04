use miette::{Context, Result};

use clap::Parser;
use x4::engine::args;
use x4::engine::configuration::parse_pipeline;
use x4::helpers::{run_app, setup_logging};

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
