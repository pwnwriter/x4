mod args;
mod corex;
mod parser;

use clap::Parser;
use corex::inspect;
use miette::{Context, Result};
use parser::parse_pipeline;
use std::path::Path;

fn main() -> Result<()> {
    let cli = args::Cli::parse();
    let pipeline_file_path = cli.pipeline;
    let path = Path::new(&pipeline_file_path);
    let pipeline = parse_pipeline(path).context("Pipeline parsing failed")?;

    for server in pipeline.servers {
        println!("{:?}", server.name);
        println!("{:?}", server.commands[1]);

        let private_key_path = server
            .resolve_private_key()
            .context("Failed to resolve private key")?;
        println!("{:?}", private_key_path);

        let password = server.resolve_password();

        println!("{:?}", password);

        println!("{}", server.port);
    }

    if cli.inspect {
        inspect::inspect_available();
    }

    Ok(())
}
