use colored::Colorize;
use miette::{Context, IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pipeline {
    pub servers: Vec<Server>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub name: String,
    pub host: String,
    #[serde(default = "default_port")]
    pub port: i64,
    pub user: String,
    #[serde(rename = "private_key")]
    pub private_key: Option<String>,
    #[serde(rename = "password")]
    pub password: Option<String>,
    pub commands: Vec<String>,
    pub description: Option<String>,
}

impl Server {
    pub fn resolve_private_key(&self) -> Result<Option<PathBuf>> {
        self.resolve_env_variable(&self.private_key)
            .map(|opt| opt.map(PathBuf::from))
    }

    pub fn resolve_password(&self) -> Result<Option<String>> {
        self.resolve_env_variable(&self.password)
    }

    fn resolve_env_variable(&self, var: &Option<String>) -> Result<Option<String>> {
        if let Some(ref value) = var {
            if value.starts_with("env:") {
                let var_name = value.strip_prefix("env:").unwrap();
                env::var(var_name)
                    .map(Some)
                    .map_err(|_| miette::miette!("Environment variable {} not found", var_name))
            } else {
                Ok(Some(value.clone()))
            }
        } else {
            Ok(None)
        }
    }
}

fn default_port() -> i64 {
    22
}

pub fn parse_pipeline(pipeline_file: &Path) -> Result<Pipeline> {
    let contents = fs::read_to_string(pipeline_file)
        .into_diagnostic()
        .wrap_err_with(|| {
            format!(
                "Cannot read {}. Please provide a valid file path.",
                pipeline_file.display().to_string().bright_cyan().bold()
            )
        })?;

    let pipeline: Pipeline = serde_json::from_str(&contents)
        .into_diagnostic()
        .wrap_err("Failed to parse JSON content")?;

    Ok(pipeline)
}
