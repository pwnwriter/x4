use crate::corex::ssh::{connect_with_password, connect_with_private_key};
use miette::{Context, Result};
use sxm::{
    corex::{args::Cli, inspect},
    parser::{PasswordRetriever, Pipeline},
};
use tracing::info;

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
