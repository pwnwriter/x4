mod args;
mod engine;
mod parser;

use clap::Parser;
use miette::{Context, Result};
use parser::parse_pipeline;
use std::path::Path;

fn main() -> Result<()> {
    let cli = args::Cli::parse();
    let pipeline_file_path = cli.pipeline_file;
    let path = Path::new(&pipeline_file_path);
    let pipeline = parse_pipeline(path).context("Pipeline parsing failed")?;

    for server in pipeline.servers {
        println!("{:?}", server.name);
    }

    let hostname = String::from("fawn.pwnwriter.xyz");
    let username = String::from("fawn");
    let port = 22;

    engine::ssh::connect_via_password(hostname, username, Some(port));

    Ok(())
}
