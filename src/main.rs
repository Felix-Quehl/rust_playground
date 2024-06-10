use std::process;

mod communication;
mod processing;

static ENDPOINT: &str = "0.0.0.0:80";

fn main() {
    let callback = processing::process_tcp_stream;
    let result = communication::communicate_via_tcp(ENDPOINT, callback);

    if let Err(e) = result {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}
