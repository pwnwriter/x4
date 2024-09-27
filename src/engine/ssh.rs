use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;

pub fn connect_via_key(host: String, port: Option<i64>, private_key: String) {
    println!("{}", host);
}

pub fn connect_via_password(host: String, user: String, port: Option<i64>) {
    let port = port.unwrap_or(22);
    let address = format!("{}:{}", host, port);

    let tcp = TcpStream::connect(address).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_agent(&user).unwrap();

    let mut channel = sess.channel_session().unwrap();
    channel.exec("mkdir -p hello").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    let _ = channel.wait_close();
    println!("{}", channel.exit_status().unwrap());
}
