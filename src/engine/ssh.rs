use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;

pub fn connect_via_password(host: String, user: String, port: Option<i64>) {
    let port = port.unwrap_or(22);
    let address = format!("{}:{}", host, port);

    let tcp = TcpStream::connect(address).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_agent(&user).unwrap();

    let mut channel = sess.channel_session().unwrap();

    channel.exec("mkdir -p hello && ls && pwd && whoami && wh && pnpm").unwrap();

    let mut stdout = String::new();
    channel.read_to_string(&mut stdout).unwrap();

    let mut stderr = String::new();
    channel.read_to_string(&mut stderr).unwrap();

    let _ = channel.wait_close();

    let exit_status = channel.exit_status().unwrap();

    if !stdout.is_empty() {
        println!("STDOUT: {}", stdout);
    }
    if !stderr.is_empty() {
        eprintln!("STDERR: {}", stderr);
    }
    println!("Exit Status: {}", exit_status);
}
