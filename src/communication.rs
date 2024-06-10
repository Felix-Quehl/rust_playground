use std::net::{TcpListener, TcpStream};

pub fn communicate_via_tcp(
    endpoint: &str,
    callback: fn(TcpStream) -> std::io::Result<()>,
) -> std::io::Result<()> {
    let listener = TcpListener::bind(endpoint)?;
    for stream in listener.incoming() {
        callback(stream?)?;
    }
    Ok(())
}
