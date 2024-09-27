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
    #[arg(short, long, default_value = "sxm.json")]
    pub pipeline_file: PathBuf,
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
