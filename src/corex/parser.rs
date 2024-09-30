use colored::Colorize;
use miette::{Context, IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

// TODO: Remove all dead code warning

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

#[allow(dead_code)]
pub trait PasswordRetriever {
    fn retrive_password(&self) -> Result<Option<String>>;
}

impl PasswordRetriever for Server {
    fn retrive_password(&self) -> Result<Option<String>> {
        if let Some(ref password) = self.password {
            if password.starts_with("cmd:") {
                let command = password.strip_prefix("cmd:").unwrap();
                return execute_command(command);
            } else if password.starts_with("env:") {
                let var_name = password.strip_prefix("env:").unwrap();
                return env::var(var_name)
                    .map(Some)
                    .map_err(|_| miette::miette!("Environment variable {} not found", var_name));
            } else {
                return Ok(Some(password.clone()));
            }
        }
        Ok(None)
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
            .map_err(|e| miette::miette!("Failed to convert stdout to string: {}", e))?;
        Ok(Some(stdout.trim().to_string()))
    } else {
        let stderr = String::from_utf8(output.stderr)
            .map_err(|e| miette::miette!("Failed to convert stderr to string: {}", e))?;
        Err(miette::miette!("Command failed: {}", stderr))
    }
}

#[allow(dead_code)]
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

fn default_port() -> i64 {
    22
}

#[allow(dead_code)]
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
