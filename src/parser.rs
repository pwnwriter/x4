use colored::Colorize;
use miette::{Context, IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

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
    pub port: i64,
    pub user: String,
    #[serde(rename = "private_key")]
    pub private_key: String,
    pub timeout: i64,
    #[serde(rename = "useSSHAgent")]
    pub use_sshagent: bool,
    pub commands: Vec<Command>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Command {
    pub description: String,
    pub command: String,
    pub env: Env,
    pub working_directory: String,
    pub output_handling: OutputHandling,
    pub error_handling: ErrorHandling,
    pub ssh_options: Vec<String>,
    pub sudo: Sudo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Env {
    #[serde(rename = "VAR1")]
    pub var1: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputHandling {
    pub save_to_file: String,
    pub return_output: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorHandling {
    pub retries: i64,
    pub log_errors: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sudo {
    pub enabled: bool,
}

pub fn parse_pipeline(pipeline_file: &Path) -> Result<Pipeline> {
    let contents = fs::read_to_string(pipeline_file)
        .into_diagnostic()
        .wrap_err_with(|| {
            format!(
                "Failed to read pipeline file: {}",
                pipeline_file.display().to_string().bold()
            )
        })?;

    let pipeline: Pipeline = serde_json::from_str(&contents)
        .into_diagnostic()
        .wrap_err("Failed to parse the JSON content into a Pipeline struct")?;

    Ok(pipeline)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pipeline_valid_json() {
        let pipeline_file_path = "examples/sshy.json";

        let pipeline =
            parse_pipeline(pipeline_file_path.as_ref()).expect("Failed to parse pipeline");

        println!("{:#?}", pipeline);
        assert_eq!(pipeline.servers.len(), 2);

        // Server 1
        assert_eq!(pipeline.servers[0].name, "server1");
        assert_eq!(pipeline.servers[0].host, "192.168.1.1");
        assert_eq!(pipeline.servers[0].port, 22);
        assert_eq!(pipeline.servers[0].user, "username");
        assert_eq!(pipeline.servers[0].private_key, "/path/to/private/key");
        assert_eq!(pipeline.servers[0].timeout, 30);
        assert_eq!(pipeline.servers[0].use_sshagent, false);
        assert_eq!(pipeline.servers[0].commands.len(), 2);
        assert_eq!(
            pipeline.servers[0].commands[0].description,
            "List directory contents"
        );
        assert_eq!(pipeline.servers[0].commands[0].command, "ls -l");
        assert_eq!(
            pipeline.servers[0].commands[0].env.var1,
            Some("value1".to_string())
        );
        assert_eq!(
            pipeline.servers[0].commands[0].working_directory,
            "/home/username"
        );
        assert_eq!(pipeline.servers[0].commands[1].description, "Show hostname");
        assert_eq!(pipeline.servers[0].commands[1].command, "cat /etc/hostname");

        // Server 2
        assert_eq!(pipeline.servers[1].name, "server2");
        assert_eq!(pipeline.servers[1].host, "192.168.1.2");
        assert_eq!(pipeline.servers[1].port, 22);
        assert_eq!(pipeline.servers[1].user, "anotheruser");
        assert_eq!(
            pipeline.servers[1].private_key,
            "/path/to/another/private/key"
        );
        assert_eq!(pipeline.servers[1].timeout, 30);
        assert_eq!(pipeline.servers[1].use_sshagent, false);
        assert_eq!(pipeline.servers[1].commands.len(), 2);
        assert_eq!(
            pipeline.servers[1].commands[0].description,
            "Display disk usage"
        );
        assert_eq!(pipeline.servers[1].commands[0].command, "df -h");
        assert_eq!(
            pipeline.servers[1].commands[0].env.var1,
            Some("value1".to_string())
        );
        assert_eq!(
            pipeline.servers[1].commands[1].description,
            "Show system uptime"
        );
        assert_eq!(pipeline.servers[1].commands[1].command, "uptime");
    }
}
