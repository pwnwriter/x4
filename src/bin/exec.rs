use miette::{Context, Result};
use sxm::{
    configuration::{PasswordRetriever, Pipeline},
    engine::{args::Cli, inspect},
    ssh::{connect_with_password, connect_with_private_key},
};
use tracing::info;

use clap::Parser;
use sxm::engine::args;
use sxm::engine::configuration::parse_pipeline;
use sxm::helpers::setup_logging;

pub fn run_app(cli: Cli, pipeline: Pipeline) -> Result<()> {
    for server in pipeline.servers {
        info!("Processing server: {}", server.name);

        // Retrieve private key path
        let private_key_path = server
            .get_private_key()
            .context("Failed to resolve private key")?
            .map(|path| path.to_string_lossy().into_owned());

        if let Some(password) = server
            .retrieve_password()
            .context("Failed to get password")?
        {
            connect_with_password(
                server.host.clone(),
                server.user.clone(),
                Some(server.port as u16), // Cast to u16 for the connect function
                password,
                server.commands.clone(),
            );
        } else if let Some(private_key) = private_key_path {
            info!("Connecting via private key");
            connect_with_private_key(
                server.host.clone(),
                server.user.clone(),
                Some(server.port as u16),
                private_key,
                server.commands.clone(),
            );
        } else {
            eprintln!(
                "No authentication method provided for server: {}",
                server.name
            );
        }
    }

    if cli.inspect {
        inspect::inspect_available();
    }

    Ok(())
}

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
