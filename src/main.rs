use server::Server;
use app::WebHandler;
use std::env;

mod entities;
mod server;
mod traits;
mod app;

fn main() {
    let root_folder = env::var("ROOT").expect("Root folder not set. Please create the environment variable ROOT.");
    let addr = String::from("127.0.0.1:8080");
    let server = Server::new(addr);
    println!("Static files folder: {}", root_folder);
    server.run(WebHandler::new(root_folder));
}
