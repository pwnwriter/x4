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
    #[arg(long, default_value = "sxm.json")]
    pub pipeline: PathBuf,

    /// Connect and request a list of identities
    #[arg(long)]
    pub inspect: bool,

    /// Check pipeline configuration
    #[arg(long)]
    pub check_pipeline: bool,
}

pub fn splash() -> String {
    let sxm_version = env!("CARGO_PKG_VERSION");

    let logo = format!(
        r#"
     ┏┓  ┳┳┓
     ┗┓┓┏┃┃┃
     ┗┛┛┗┛ ┗{}
             @pwnwriter/sxm
 
        "#,
        sxm_version
    )
    .purple();

    format!("{logo}")
}
