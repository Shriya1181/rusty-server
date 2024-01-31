use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let method = request
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap();

    match method {
        "GET" => {
            // Handle GET requests
            let name = "Mardav";

            // Check if the request is for the "/rusty" endpoint
            if request.contains("/rusty") {
                // Respond with a custom message for the "/rusty" endpoint
                let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{{\"message\": \"Welcome to the Rusty endpoint!\"}}"
                        );

                println!("Request: {}", request);

                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
                println!("Response sent!");
            } else {
                // Respond with a JSON object for the default endpoint
                let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{{\"name\": \"Welcome rustacean, {}\"}}",
                            name
                        );

                println!("Request: {}", request);

                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
                println!("Response sent!");
            }
        }

        _ => {
            let response = "HTTP/1.1 501 Not Implemented\r\n\r\nMethod Not Implemented";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream);
    }
}
