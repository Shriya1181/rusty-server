use std::{
    fs,
    thread,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};
 
pub mod thread_pool;
use thread_pool::ThreadPool;

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

    println!("Request: {}", request);

    match method {
        "GET" => {
            // Handle GET requests
            let name = "Shriya"; // Replace this with your name
            let response:String;

            if request.contains("/rusty") {
                response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{{\"message\": \"Welcome to the Rusty endpoint!\"}}"
                        );
            }
            // else if request.contains("/sleep") {
            //     std::thread::sleep(std::time::Duration::from_secs(5));
            //     response = format!(
            //         "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{{\"message\": \"I had a sweet nap!\"}}"
            //     );
            // }
            // create a new endpoint with /your-name, and send a custom message, "your-name is now a rustacean!"
            else {
                response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{{\"name\": \"Welcome rustacean, {}\"}}",
                            name
                        );
            }
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
            println!("Response sent!");
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
    // let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // thread::spawn(||{
            handle_client(stream);
        // });
        // pool.execute(||{
        //     handle_client(stream);
        // });
    }
}

// Rocket Server(demo): comment out the above code to run this, and uncomment the below code to get it running

// #[macro_use]
// extern crate rocket;

// use rocket::serde::json::Json;
// use rocket::serde::json::Value;
// use serde_json::json;

// #[get("/")]
// fn index() -> &'static str {
//     "Server Running Successfully!"
// }

// #[get("/name/<person>")]
// fn add_person(person: &str) -> Value {
//     json!({
//         "message": format!("{person}, is now a rustacean!")
//     })
// }

// #[launch]
// pub fn rocket() -> _ {
//     rocket::build().mount("/", routes![index, add_person])
// }
