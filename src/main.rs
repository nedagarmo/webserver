use server::Server;
use app::WebHandler;

mod entities;
mod server;
mod traits;
mod app;

fn main() {
    let addr = String::from("127.0.0.1:8080");

    let server = Server::new(addr);
    server.run(WebHandler);
}
