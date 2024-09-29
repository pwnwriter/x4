use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;
use std::path::Path;
use tracing::{error, info};

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
    let port = port.unwrap_or(22);
    let address = format!("{}:{}", host, port);

    match TcpStream::connect(&address) {
        Ok(tcp) => {
            let mut session = Session::new().expect("Failed to create session");
            session.set_tcp_stream(tcp);

            if let Err(err) = session.handshake() {
                error!("Handshake failed: {}", err);
                return;
            }

            info!(
                "Attempting to authenticate using private key: {}",
                private_key
            );
            let key_path = Path::new(&private_key);
            if session
                .userauth_pubkey_file(&user, None, key_path, None)
                .is_err()
            {
                error!("Private key authentication failed.");
                return;
            }

            info!("Authenticated successfully using private key.");
            execute_commands(&mut session, commands);
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
    let port = port.unwrap_or(22);
    let address = format!("{}:{}", host, port);

    match TcpStream::connect(&address) {
        Ok(tcp) => {
            let mut session = Session::new().expect("Failed to create session");
            session.set_tcp_stream(tcp);

            if let Err(err) = session.handshake() {
                error!("Handshake failed: {}", err);
                return;
            }

            info!("Attempting to authenticate using password.");
            if session.userauth_password(&user, &password).is_err() {
                error!("Password authentication failed.");
                return;
            }

            info!("Authenticated successfully using password.");
            execute_commands(&mut session, commands);
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
pub fn execute_commands(session: &mut Session, commands: Vec<String>) {
    for command in commands {
        let mut channel = session.channel_session().expect("Failed to open channel");

        // Execute the command
        if let Err(err) = channel.exec(&command) {
            error!("Failed to execute command: {}", err);
            continue;
        }

        let mut stdout = String::new();
        let mut stderr = String::new();

        // Read stdout
        channel
            .read_to_string(&mut stdout)
            .expect("Failed to read stdout.");

        // Read stderr (non-blocking)
        let mut error_channel = channel.stderr();
        error_channel
            .read_to_string(&mut stderr)
            .expect("Failed to read stderr.");

        // Wait for the channel to close
        channel.send_eof().expect("Failed to send EOF");
        channel
            .wait_close()
            .expect("Failed to wait for channel close");

        let exit_status = channel.exit_status().expect("Failed to get exit status");

        // Print outputs and exit status
        if !stdout.is_empty() {
            println!("STDOUT: {}", stdout);
        }
        if !stderr.is_empty() {
            eprintln!("STDERR: {}", stderr);
        }

        println!("Exit Status for '{}': {}", command, exit_status);
    }
}
