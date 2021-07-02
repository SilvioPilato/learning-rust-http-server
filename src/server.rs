use std::time::UNIX_EPOCH;
use std::time::SystemTime;
use crate::WebsiteHandler;
use std::net::TcpStream;
use super::http::ParseError;
use super::http::StatusCode;
use super::http::Response;
use std::net::TcpListener;
use std::convert::TryFrom;
use crate::http::Request;
use std::io::{Read};
use std::env;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
#[derive(Copy, Clone)]
pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Server {}
    } 

    pub fn run(self, addr: String) {
        println!("Server running at address: {}", addr);
        let listener = TcpListener::bind(addr).unwrap();
        loop {
            match listener.accept() {
                Ok((stream, _)) => {
                    std::thread::spawn(move || {
                        let start = SystemTime::now();
                        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
                        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
                        self.handle_connection(stream, &mut WebsiteHandler::new(public_path));
                        match start.elapsed() {
                            Ok(elapsed) => {
                                println!("Request served in {} ms", elapsed.as_millis());
                            }
                            Err(e) => {
                                println!("Error: {:?}", e);
                            }
                        }
                    });
                }
                Err(e) => println!("Failed to enstablish a connection: {}", e),
            }
        }
    }
    
    fn handle_connection(&self, mut stream: TcpStream, handler: &mut impl Handler) {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(_) => {
                println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                let response = match Request::try_from(&buffer[..]) {
                    Ok(request) => {
                        handler.handle_request(&request)
                    },
                    Err(e) => {
                        handler.handle_bad_request(&e)
                    }
                };
                if let Err(e) = response.send(&mut stream) {
                    println!("Failed to send response: {}", e);
                }
            }
            Err(e) => println!("Failed to read from connection: {}", e)
        }
    }

}
