use std::path::PathBuf;

use clap::Parser;
use colored::Colorize;

#[derive(Debug, Parser, Clone)]
#[command(
    author,
    version,
    about = splash(),
    propagate_version = true,
    arg_required_else_help = true
)]
pub struct Cli {
    /// Path to your pipeline file
    #[arg(short, long)]
    pub pipeline: Option<PathBuf>,

    /// Connect and request a list of identities
    #[arg(long)]
    pub inspect: bool,

    /// Check the configuration of the specified pipeline.
    #[arg(long)]
    pub check_pipeline: bool,
}

pub fn splash() -> String {
    let x4_version = env!("CARGO_PKG_VERSION");

    let logo = format!(
        r#"
      ┏┓
    ┓┏┃┃
    ┛┗┗╋
        {}
             @pwnwriter/x4
 
        "#,
        x4_version
    )
    .purple();

    format!("{logo}")
}
