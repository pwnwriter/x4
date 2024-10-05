use colored::Colorize;
use miette::{Context, IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
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
    pub user: String,

    #[serde(default = "default_port")]
    pub port: i64,

    #[serde(rename = "private_key")]
    pub private_key: Option<String>,

    #[serde(rename = "password")]
    pub password: Option<String>,

    pub commands: Vec<String>,
    pub description: Option<String>,
}

pub trait PasswordRetriever {
    fn retrieve_password(&self) -> Result<Option<String>>;
}

impl PasswordRetriever for Server {
    fn retrieve_password(&self) -> Result<Option<String>> {
        match &self.password {
            Some(password) => match password.strip_prefix("cmd:") {
                Some(command) => execute_command(command),
                None => match password.strip_prefix("env:") {
                    Some(var_name) => env::var(var_name).map(Some).map_err(|_| {
                        miette::miette!("Environment variable {} not found", var_name)
                    }),
                    None => Ok(Some(password.clone())),
                },
            },
            None => Ok(None),
        }
    }
}

fn execute_command(command: &str) -> Result<Option<String>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .into_diagnostic()
        .wrap_err("Failed to execute command")?;

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout)
            .map_err(|e| miette::miette!("Failed to convert stdout to string: {e}"))?;
        Ok(Some(stdout.trim().to_string()))
    } else {
        let stderr = String::from_utf8(output.stderr)
            .map_err(|e| miette::miette!("Failed to convert stderr to string: {e}"))?;
        Err(miette::miette!("Command failed: {}", stderr))
    }
}

impl Server {
    pub fn get_private_key(&self) -> Result<Option<PathBuf>> {
        self.get_env_variable(&self.private_key)
            .map(|opt| opt.map(PathBuf::from))
    }

    fn get_env_variable(&self, var: &Option<String>) -> Result<Option<String>> {
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

pub fn default_port() -> i64 {
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
