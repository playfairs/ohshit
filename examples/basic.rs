use std::io::{Read, Write};
use std::net::TcpListener;

use ohshit::{Diagnostic, Location, info, ohshit};

fn main() {
    ohshit::init();

    let address = "127.0.0.1:8000";
    match TcpListener::bind(address) {
        Ok(listener) => {
            info!(host = "127.0.0.1", port = 8000; "listening on {}", address);

            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    let _ = stream.read(&mut buffer);

                    let response = b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK";
                    let _ = stream.write_all(response);

                    info!(client = "127.0.0.1", status = "ok"; "handled a request on {}", address);
                }
                Err(error) => {
                    let diagnostic = Diagnostic::new(format!("Could not accept a connection on {address}"))
                        .with_cause(error.to_string())
                        .with_reason("The runtime failed while awaiting a client connection.")
                        .with_action("check the socket state and retry the listener")
                        .with_location(Location::new(file!(), line!()));

                    ohshit!(host = "127.0.0.1", port = 8000; "accept failed on {}", address);
                    eprintln!("{}", diagnostic.render());
                }
            }
        }
        Err(error) => {
            let diagnostic = Diagnostic::new(format!("Could not bind to {address}"))
                .with_cause(error.kind().to_string())
                .with_reason(error.to_string())
                .with_action("stop the existing listener or choose another port")
                .with_location(Location::new(file!(), line!()));

            ohshit!(host = "127.0.0.1", port = 8000; "bind failed for {}", address);
            eprintln!("{}", diagnostic.render());
        }
    }
}
