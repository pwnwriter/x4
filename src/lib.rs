//! sxm: Execute shell command, (down/up)load files to a server via ssh protocol
//!
//! Copyright (c) pwnwriter <hey@pwnwriter.xyz>

pub mod corex;
pub use corex::{args, parser, ssh};
