use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;
use tracing::{error, info};

#[allow(dead_code)]
pub fn connect_via_password(host: String, user: String, port: Option<i64>) {
    let port = port.unwrap_or(22);
    let address = format!("{}:{}", host, port);

    let tcp = TcpStream::connect(address).expect("Failed to connect");
    let mut sess = Session::new().expect("Failed to create session");
    sess.set_tcp_stream(tcp);
    sess.handshake().expect("Failed to handshake");
    sess.userauth_agent(&user).expect("Authentication failed");

    let mut channel = sess.channel_session().expect("Failed to open channel");

    let command = "mkdir -p hello && ls && pwd && whoami && whoami && ping gnu.org -c 5";

    info!("Executing command: {}", command);

    channel.exec(command).expect("Failed to execute command");

    let mut stdout = String::new();
    let mut stderr = String::new();

    channel
        .read_to_string(&mut stdout)
        .expect("Failed to read stdout");

    let mut error_channel = channel.stderr();

    error_channel
        .read_to_string(&mut stderr)
        .expect("Failed to read stderr");

    let _ = channel.wait_close();
    let exit_status = channel.exit_status().expect("Failed to get exit status");

    if !stdout.is_empty() {
        info!("STDOUT: {}", stdout);
    }
    if !stderr.is_empty() {
        error!("STDERR: {}", stderr);
    }

    info!("Exit Status: {}", exit_status);
}
