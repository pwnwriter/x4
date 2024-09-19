use std::path::PathBuf;

use clap::Parser;
use colored::Colorize;

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about = splash(),
    propagate_version = true,
)]
pub struct Cli {
    /// Pipeline file path
    #[arg(short, long, default_value = "sshy.json")]
    pub pipeline_file: PathBuf,
}

pub fn splash() -> String {
    let hxn_version = env!("CARGO_PKG_VERSION");

    let logo = format!(
        r#"
      ┏┓┏┓┓┏  
      ┗┓┗┓┣┫┓┏
      ┗┛┗┛┛┗┗┫
             ┛{}
             @pwnwriter/sshy
 "#,
        hxn_version
    )
    .purple();

    format!("{logo}")
}
