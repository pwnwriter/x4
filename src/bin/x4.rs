use clap::Parser;
use miette::{Context, Result};
use tracing::error;
use x4::engine::args;
use x4::engine::configuration::parse_pipeline;
use x4::helpers::{run_app, setup_logging};
use x4::interaction::validate_pipeline_from_file;

fn main() -> Result<()> {
    let cli = args::Cli::parse();

    setup_logging()?;

    match (cli.check_pipeline.as_ref(), cli.pipeline.as_ref()) {
        (Some(check_path), None) => {
            validate_pipeline_from_file(check_path)?;
        }
        (None, Some(pipeline_path)) => {
            let pipeline = parse_pipeline(pipeline_path).context("Failed to parse the pipeline")?;
            run_app(cli.clone(), pipeline)?;
        }
        (Some(_), Some(_)) => {
            error!("Please specify either --check-pipeline or --pipeline, not both.");
        }
        (None, None) => {
            error!("No pipeline file path provided.");
        }
    }

    Ok(())
}
