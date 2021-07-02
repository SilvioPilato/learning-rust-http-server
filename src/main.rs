#![allow(dead_code)]

use crate::website_handler::WebsiteHandler;
use server::Server;

mod server;
mod http;
mod website_handler;

fn main() {
    let server: Server = Server::new();
    server.run("127.0.0.1:8080".to_string());
}
