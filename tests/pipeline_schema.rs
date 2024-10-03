use miette::Result;
use std::path::Path;
use sxm::corex::parser::parse_pipeline;

#[test]
fn test_parse_pipeline() -> Result<()> {
    const EXPECTED_SERVER_NAME: &str = "ec2";
    const EXPECTED_SERVER_HOST: &str = "fawn.pwnwriter.xyz";
    const EXPECTED_SERVER_USER: &str = "fawn";
    const EXPECTED_SERVER_PORT: i64 = 22;
    const EXPECTED_COMMANDS: [&str; 2] = ["ls -l", "cat /etc/hostname"];
    const EXPECTED_DESCRIPTION: &str =
        "using a private key for SSH authentication and default 22 port";
    const EXPECTED_PASSWORD: &str = "env:wolf_pass";

    let pipeline_path = "examples/test_schema.json";
    let pipeline_file_path = Path::new(pipeline_path);

    let pipeline = parse_pipeline(pipeline_file_path)?;

    // Validate number of servers
    assert_eq!(pipeline.servers.len(), 4);

    // Validate first server
    let first_server = &pipeline.servers[0];
    assert_eq!(first_server.name, EXPECTED_SERVER_NAME);
    assert_eq!(first_server.host, EXPECTED_SERVER_HOST);
    assert_eq!(first_server.user, EXPECTED_SERVER_USER);
    assert_eq!(first_server.port, EXPECTED_SERVER_PORT);
    assert_eq!(first_server.commands.len(), EXPECTED_COMMANDS.len());

    // Validate commands
    for (expected, actual) in EXPECTED_COMMANDS.iter().zip(&first_server.commands) {
        assert_eq!(expected, actual);
    }

    // Validate optional fields
    assert_eq!(
        first_server.description.as_deref(),
        Some(EXPECTED_DESCRIPTION)
    );
    assert_eq!(first_server.password.as_deref(), Some(EXPECTED_PASSWORD));
    assert!(first_server.private_key.is_none()); // assuming private_key is not present in the JSON

    Ok(())
}
