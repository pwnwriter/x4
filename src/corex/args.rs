use std::path::PathBuf;

use clap::{Args, Parser};
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

    #[command(flatten)]
    pub file: File,
}

#[derive(Debug, Args, Clone)]
#[group(required = false, multiple = false)]
pub struct File {
    /// Local file path to upload/download
    #[arg(short, long)]
    pub from: Option<String>,

    /// Remote path to the file
    #[arg(short, long)]
    pub to: Option<String>,
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
