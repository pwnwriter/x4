use miette::{Context, Result};
use sxm::corex::{args::Cli, inspect, parser::Pipeline, ssh};

pub fn run_app(pipeline: Pipeline, cli: Cli) -> Result<()> {
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

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    ssh::connect_via_password("44.202.26.95".to_string(), "fawn".to_string(), Some(22));

    Ok(())
}
