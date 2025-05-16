use std::io::{Read};
use std::net::TcpListener;
use super::traits::Handler;
use super::entities::{Request};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        let listener = TcpListener::bind(&self.addr).expect("Failed to bind server port");
        println!("Simple server is running on {}", self.addr);

        loop {
            match listener.accept() {
                Ok((mut stream, client_address)) => {
                    println!("Client {} connected", client_address);
                    let mut buffer = [0; 1024];
                    stream.read(&mut buffer).expect("Failed to read message");
                    println!("Plain message received: {}", String::from_utf8_lossy(&buffer));
                    let response = match Request::try_from(&buffer[..]) {
                        Ok(request) => {
                            dbg!(&request);
                            handler.handle_request(&request)
                        },
                        Err(error) => handler.handle_bad_request(&error),
                    };

                    if let Err(e) = response.send(&mut stream) {
                        dbg!(&e);
                        println!("Failed to send response {}", e);
                    }
                },
                Err(e) => println!("Failed to establish a new connection: {}", e),
            }
        }
    }
}