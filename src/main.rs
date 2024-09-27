mod corex;
mod exec;

use clap::Parser;
use corex::parser::parse_pipeline;
use corex::{args, inspect, ssh};
use miette::{Context, Result};
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

    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::TRACE)
    //     .init();

    ssh::connect_via_password("44.202.26.95".to_string(), "fawn".to_string(), Some(22));
    Ok(())
}
