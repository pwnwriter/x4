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
    pub commands: Vec<Command>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Command {
    pub description: String,
    pub command: String,
}

pub fn parse_pipeline(pipeline_file: &Path) -> Result<Pipeline> {
    let contents = fs::read_to_string(pipeline_file)
        .into_diagnostic()
        .context(format!(
            "Failed to read the pipeline JSON file at: {:?}",
            pipeline_file
        ))?;

    let pipeline: Pipeline = serde_json::from_str(&contents)
        .into_diagnostic()
        .context("Failed to parse the JSON content into a Pipeline struct")?;

    Ok(pipeline)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    // create a temporary JSON file
    fn create_temp_file(content: &str) -> PathBuf {
        let temp_file = std::env::temp_dir().join("test_pipeline.json");
        let mut file = File::create(&temp_file).expect("unable to create temp file");
        write!(file, "{}", content).expect("unable to write to temp file");
        temp_file
    }

    #[test]
    fn test_parse_pipeline_valid_json() {
        // Create a temporary JSON file with valid content
        let json_content = r#"
        {
          "servers": [
            {
              "name": "server1",
              "host": "192.168.1.1",
              "port": 22,
              "user": "username",
              "private_key": "/path/to/private/key",
              "commands": [
                {
                  "description": "List directory contents",
                  "command": "ls -l"
                },
                {
                  "description": "Show hostname",
                  "command": "cat /etc/hostname"
                }
              ]
            },
            {
              "name": "server2",
              "host": "192.168.1.2",
              "port": 22,
              "user": "anotheruser",
              "private_key": "/path/to/another/private/key",
              "commands": [
                {
                  "description": "Display disk usage",
                  "command": "df -h"
                },
                {
                  "description": "Show system uptime",
                  "command": "uptime"
                }
              ]
            }
          ]
        }"#;

        let temp_file = create_temp_file(json_content);

        // Parse the pipeline
        let pipeline = parse_pipeline(&temp_file).expect("failed to parse pipeline");

        println!("{:#?}", pipeline);
        assert_eq!(pipeline.servers.len(), 2);

        // server 1
        assert_eq!(pipeline.servers[0].name, "server1");
        assert_eq!(pipeline.servers[0].host, "192.168.1.1");
        assert_eq!(pipeline.servers[0].port, 22);
        assert_eq!(pipeline.servers[0].user, "username");
        assert_eq!(pipeline.servers[0].private_key, "/path/to/private/key");
        assert_eq!(pipeline.servers[0].commands.len(), 2);
        assert_eq!(
            pipeline.servers[0].commands[0].description,
            "List directory contents"
        );
        assert_eq!(pipeline.servers[0].commands[0].command, "ls -l");
        assert_eq!(pipeline.servers[0].commands[1].description, "Show hostname");
        assert_eq!(pipeline.servers[0].commands[1].command, "cat /etc/hostname");

        // server 2
        assert_eq!(pipeline.servers[1].name, "server2");
        assert_eq!(pipeline.servers[1].host, "192.168.1.2");
        assert_eq!(pipeline.servers[1].port, 22);
        assert_eq!(pipeline.servers[1].user, "anotheruser");
        assert_eq!(
            pipeline.servers[1].private_key,
            "/path/to/another/private/key"
        );
        assert_eq!(pipeline.servers[1].commands.len(), 2);
        assert_eq!(
            pipeline.servers[1].commands[0].description,
            "Display disk usage"
        );
        assert_eq!(pipeline.servers[1].commands[0].command, "df -h");
        assert_eq!(
            pipeline.servers[1].commands[1].description,
            "Show system uptime"
        );
        assert_eq!(pipeline.servers[1].commands[1].command, "uptime");
    }
}
