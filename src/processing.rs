use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

pub fn process_tcp_stream(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;

    // Convert the request bytes to a string
    let request = str::from_utf8(&buffer).unwrap_or("");
    println!("Received request: {}", request);

    // Parse the HTTP request (very basic parsing)
    let mut lines = request.lines();
    if let Some(request_line) = lines.next() {
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() == 3 {
            let method = parts[0];
            let path = parts[1];
            let _http_version = parts[2];

            // Handle different HTTP methods (only GET is handled in this example)
            if method == "GET" {
                let response = if path == "/" {
                    "HTTP/1.0 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, World!"
                } else {
                    "HTTP/1.0 404 Not Found\r\nContent-Type: text/plain\r\n\r\nPage not found"
                };

                stream.write_all(response.as_bytes())?;
                stream.flush()?;
            } else {
                // Method not allowed
                let response = "HTTP/1.0 405 Method Not Allowed\r\nContent-Type: text/plain\r\n\r\nMethod not allowed";
                stream.write_all(response.as_bytes())?;
                stream.flush()?;
            }
        } else {
            // Bad request
            let response =
                "HTTP/1.0 400 Bad Request\r\nContent-Type: text/plain\r\n\r\nBad request";
            stream.write_all(response.as_bytes())?;
            stream.flush()?;
        }
    } else {
        // Empty request
        let response = "HTTP/1.0 400 Bad Request\r\nContent-Type: text/plain\r\n\r\nBad request";
        stream.write_all(response.as_bytes())?;
        stream.flush()?;
    }

    Ok(())
}
