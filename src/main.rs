mod parser;
use std::path::Path;

use parser::parse_pipeline;

fn main() {
    let path = Path::new("examples/sshy.json");
    match parse_pipeline(path) {
        Ok(pipeline) => {
            for server in pipeline.servers {
                println!("{:?}", server.name);
            }
        }
        Err(e) => eprintln!("Error parsing pipeline: {}", e),
    }
}
