use std::env;
use std::process;

mod communication;
mod processing;

const ARG_COUNT: usize = 2;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != ARG_COUNT {
        eprintln!("Usage: {} <127.0.0.1:8080>", args[0]);
        std::process::exit(1);
    } else {
        let tcp_endpoint = &args[1];
        let callback = processing::process_tcp_stream;
        let result = communication::communicate_via_tcp(tcp_endpoint, callback);

        if let Err(e) = result {
            eprintln!("error: {}", e);
            process::exit(1);
        }
    }
}
