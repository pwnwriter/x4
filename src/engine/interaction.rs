use crate::configuration::Pipeline;
use colored::Colorize;
use miette::IntoDiagnostic;
use miette::{Result, WrapErr};
use ssh2::Session;
use std::fs;
use std::io::Read;
use std::net::TcpStream;
use std::path::Path;
use tracing::{error, info};

// TODO: Add more papeline checks

/// Connects to a remote server using SSH with a private key.
///
/// # Arguments
///
/// * `host` - The hostname or IP address of the server.
/// * `user` - The username for SSH authentication.
/// * `port` - Optional port number; defaults to 22 if not provided.
/// * `private_key` - The path to the private key file.
/// * `commands` - A vector of commands to run.
pub fn connect_with_private_key(
    host: String,
    user: String,
    port: Option<u16>,
    private_key: String,
    commands: Vec<String>,
) {
    let port = port.unwrap_or(22); // Use the default SSH port if none is provided.
    let address = format!("{}:{}", host, port); // Construct the address string.

    match TcpStream::connect(&address) {
        Ok(tcp) => {
            let mut session = Session::new().expect("Failed to create session"); // Create a new SSH session.
            session.set_tcp_stream(tcp); // Set the TCP stream for the session.

            // Perform the SSH handshake.
            if let Err(err) = session.handshake() {
                error!("Handshake failed: {}", err);
                return;
            }

            info!(
                "Attempting to authenticate using private key: {}",
                private_key
            );
            let key_path = Path::new(&private_key);
            // Authenticate using the provided private key.
            if session
                .userauth_pubkey_file(&user, None, key_path, None)
                .is_err()
            {
                error!("Private key authentication failed.");
                return;
            }

            info!("Authenticated successfully using private key.");
            execute_commands(&mut session, commands); // Execute the provided commands.
        }
        Err(err) => {
            error!("Failed to connect to {}: {}", address, err);
        }
    }
}

/// Connects to a remote server using SSH with a password.
///
/// # Arguments
///
/// * `host` - The hostname or IP address of the server.
/// * `user` - The username for SSH authentication.
/// * `port` - Optional port number; defaults to 22 if not provided.
/// * `password` - The password for SSH authentication.
/// * `commands` - A vector of commands to run.
pub fn connect_with_password(
    host: String,
    user: String,
    port: Option<u16>,
    password: String,
    commands: Vec<String>,
) {
    let port = port.unwrap_or(22); // Use the default SSH port if none is provided.
    let address = format!("{}:{}", host, port); // Construct the address string.

    match TcpStream::connect(&address) {
        Ok(tcp) => {
            let mut session = Session::new().expect("Failed to create session"); // Create a new SSH session.
            session.set_tcp_stream(tcp); // Set the TCP stream for the session.

            // Perform the SSH handshake.
            if let Err(err) = session.handshake() {
                error!("Handshake failed: {}", err);
                return;
            }

            info!("Attempting to authenticate using password.");
            // Authenticate using the provided password.
            if session.userauth_password(&user, &password).is_err() {
                error!("Password authentication failed.");
                return;
            }

            info!("Authenticated successfully using password.");
            execute_commands(&mut session, commands); // Execute the provided commands.
        }
        Err(err) => {
            error!("Failed to connect to {}: {}", address, err);
        }
    }
}

/// Executes a list of commands on the connected SSH session.
///
/// # Arguments
///
/// * `session` - A mutable reference to the SSH session.
/// * `commands` - A vector of commands to run.
///
/// This function executes each command sequentially, captures the standard output
/// and standard error, and then calls `pretty_print` to format the results.
pub fn execute_commands(session: &mut Session, commands: Vec<String>) {
    for command in commands {
        let mut channel = session.channel_session().expect("Failed to open channel"); // Open a channel for the command.

        // Execute the command on the remote server.
        if let Err(err) = channel.exec(&command) {
            error!("Failed to execute command: {}", err);
            continue; // Skip to the next command on failure.
        }

        let mut stdout = String::new();
        let mut stderr = String::new();

        // Read standard output from the command.
        channel
            .read_to_string(&mut stdout)
            .expect("Failed to read stdout.");

        // Read standard error (non-blocking).
        let mut error_channel = channel.stderr();
        error_channel
            .read_to_string(&mut stderr)
            .expect("Failed to read stderr.");

        // Wait for the channel to close.
        channel.send_eof().expect("Failed to send EOF");
        channel
            .wait_close()
            .expect("Failed to wait for channel close");

        // Get the exit status of the command.
        let exit_status = channel.exit_status().expect("Failed to get exit status");

        // Pretty print outputs.
        pretty_print(command, exit_status, stdout, stderr);
    }
}

/// Pretty prints the output of an executed command in a structured format.
///
/// # Arguments
///
/// * `command` - The command that was executed.
/// * `exit_status` - The exit status of the command.
/// * `stdout` - The standard output generated by the command.
/// * `stderr` - The standard error generated by the command.
///
/// This function formats the output for readability, displaying the command, its
/// exit status, and the results of the execution (both stdout and stderr).
pub fn pretty_print(command: String, exit_status: i32, stdout: String, stderr: String) {
    println!("Command: {}", command.bold());
    println!("  ├── Exit Status: {}", exit_status.to_string().green());

    if !stdout.is_empty() {
        println!("  ├── STDOUT:");
        for line in stdout.lines() {
            println!("  │   {}", line.green());
        }
    } else {
        println!("  ├── STDOUT: {}", "(no output)".yellow());
    }

    if !stderr.is_empty() {
        println!("  └── STDERR:");
        for line in stderr.lines() {
            println!("      {}", line.red());
        }
    } else {
        println!("  └── STDERR: {}", "(no output)".yellow());
    }
}

/// Validates the `Pipeline` configuration from a JSON file.
///
/// This function reads the pipeline configuration from the specified JSON file,
/// parses it into a `Pipeline` struct, and then checks that each `Server`
/// in the `Pipeline` contains the required fields:
/// - `name`: A unique identifier for the server.
/// - `host`: The hostname or IP address of the server.
/// - `user`: The username for authentication.
/// - `port`: The port number for the server.
///
/// Additionally, it verifies that:
/// - No two servers share the same name.
/// - The `host` field contains a valid hostname or IP address.
/// - The `port` field is a valid integer within the range 1-65535.
///
/// If any validation fails, an error is returned with a descriptive message
/// using the `miette` crate to provide context.
///
/// # Parameters
///
/// - `pipeline_file`: The path to the JSON file containing the pipeline configuration.
///
/// # Returns
///
/// Returns `Result<()>`, which is `Ok(())` if all validations pass, or an error
/// if any validation fails.
///
/// # Errors
///
/// This function may return errors such as:
/// - Issues with reading or parsing the file.
/// - Missing required fields (name, host, user, or port).
/// - Duplicate server names.
/// - Invalid host formats.
/// - Invalid port number.
pub fn validate_pipeline_from_file(pipeline_file: &Path) -> Result<()> {
    // Read the contents of the JSON file into a string.
    let contents = fs::read_to_string(pipeline_file)
        .into_diagnostic()
        .wrap_err_with(|| {
            format!(
                "Cannot read {}. Please provide a valid file path.",
                pipeline_file.display()
            )
        })?;

    let pipeline: Pipeline = serde_json::from_str(&contents)
        .into_diagnostic()
        .wrap_err("Failed to parse JSON content")?;

    // Validate the pipeline configuration.
    validate_pipeline(&pipeline)?;

    // Print a success message if validation passes.
    println!("{}", "Pipeline validation succeeded!".green().bold());

    Ok(())
}

/// Validates the given `Pipeline` configuration.
///
/// # Arguments
///
/// * `pipeline` - A reference to the `Pipeline` struct to validate.
///
/// # Returns
///
/// Returns `Result<()>`, which is `Ok(())` if all validations pass, or an error
/// if any validation fails.
pub fn validate_pipeline(pipeline: &Pipeline) -> Result<()> {
    let mut seen_names = std::collections::HashSet::new(); // To track unique server names.

    println!("{}", "Validating Pipeline Configuration:".bold().green());

    for server in &pipeline.servers {
        // Check if required fields are present.

        if server.name.is_empty() {
            return Err(miette::miette!(
                "❌ Server name is required for one of the servers."
            ));
        }
        println!("  ✔️ Server name: '{}'", server.name.bold());

        if server.host.is_empty() {
            return Err(miette::miette!(
                "❌ Host is required for server '{}'.",
                server.name
            ));
        }
        println!("  ✔️ Host: '{}'", server.host.bold());

        if server.user.is_empty() {
            return Err(miette::miette!(
                "❌ User is required for server '{}'.",
                server.name
            ));
        }
        println!("  ✔️ User: '{}'", server.user.bold());

        if server.port < 1 || server.port > 65535 {
            return Err(miette::miette!(
                "❌ Invalid port number for server '{}'. It must be between 1 and 65535.",
                server.name
            ));
        }
        println!("  ✔️ Port: '{}'", server.port.to_string().bold());

        // Check for duplicate server names.
        if !seen_names.insert(server.name.clone()) {
            return Err(miette::miette!(
                "❌ Duplicate server name found: '{}'. Each server must have a unique name.",
                server.name
            ));
        }
        println!("  ✔️ Unique server name: '{}'", server.name.bold());

        // Validate the host format (basic validation).
        if !is_valid_host(&server.host) {
            return Err(miette::miette!(
                "❌ Invalid host format for server '{}': '{}'.",
                server.name,
                server.host
            ));
        }
        println!("  ✔️ Valid host format for server '{}'", server.name.bold());
    }

    println!("{}", "All validations passed! 🎉".bold().green());
    Ok(())
}

/// Validates if the given host is a valid hostname or IP address.
///
/// # Arguments
///
/// * `host` - The hostname or IP address to validate.
///
/// # Returns
///
/// Returns `true` if the host is valid, otherwise `false`.
fn is_valid_host(host: &str) -> bool {
    // A simple check for a valid IP address or hostname.
    host.parse::<std::net::IpAddr>().is_ok() || host.contains('.')
}
