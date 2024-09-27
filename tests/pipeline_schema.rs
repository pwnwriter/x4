use miette::Result;
use std::path::Path;
use sxm::corex::parser::parse_pipeline;

#[test]
fn test_parse_pipeline() -> Result<()> {
    // Define the path to the JSON file
    let pipeline_file_path = Path::new("./examples/sxm.json");

    // Call the function to parse the pipeline
    let pipeline = parse_pipeline(pipeline_file_path)?;

    assert_eq!(pipeline.servers.len(), 4);

    let first_server = &pipeline.servers[0];
    assert_eq!(first_server.name, "ec2");
    assert_eq!(first_server.host, "fawn.pwnwriter.xyz");
    assert_eq!(first_server.user, "fawn");
    assert_eq!(first_server.port, 22);
    assert_eq!(first_server.commands.len(), 2);

    // Check the commands of the first server
    assert_eq!(first_server.commands[0], "ls -l");
    assert_eq!(first_server.commands[1], "cat /etc/hostname");

    Ok(())
}
