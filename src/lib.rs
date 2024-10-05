//! x4: Execute shell commands to a server via ssh protocol
//!
//! Copyright (c) pwnwriter <hey@pwnwriter.xyz>

pub mod engine;
pub use engine::{args, configuration, interaction};

pub mod helpers {
    use miette::Result;
    use std::env;

    use crate::{
        configuration::{PasswordRetriever, Pipeline},
        engine::args::Cli,
        interaction::{connect_with_password, connect_with_private_key},
    };
    use miette::Context;
    use tracing::info;

    pub fn setup_logging() -> Result<()> {
        if env::var("RUST_LIB_BACKTRACE").is_err() {
            env::set_var("RUST_LIB_BACKTRACE", "1")
        }

        if env::var("RUST_LOG").is_err() {
            env::set_var("RUST_LOG", "info")
        }

        tracing_subscriber::fmt::fmt().init();

        Ok(())
    }

    pub fn run_app(_cli: Cli, pipeline: Pipeline) -> Result<()> {
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
                tracing::error!(
                    "No authentication method provided for server: {}",
                    server.name
                );
            }
        }

        Ok(())
    }
}
