//! sxm: Execute shell commands to a server via ssh protocol
//!
//! Copyright (c) pwnwriter <hey@pwnwriter.xyz>

pub mod engine;
pub use engine::{args, configuration, ssh};

pub mod helpers {
    use miette::Result;
    use std::env;

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
}
