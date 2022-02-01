use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;

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
    let mut file = File::open(pipeline_file).expect("unable to find or open the JSON file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("unable to read the JSON file");

    let pipeline: Pipeline = serde_json::from_str(&contents)?;

    Ok(pipeline)
}
