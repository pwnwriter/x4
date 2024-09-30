//! sxm: Execute shell command, (down/up)load files to a server via ssh protocol
//!
//! Copyright (c) pwnwriter <hey@pwnwriter.xyz>

pub mod corex;
pub use corex::{args, parser, ssh};
use miette::Result;

pub fn setup_logging() -> Result<()> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }

    tracing_subscriber::fmt::fmt().init();

    Ok(())
}
