use std::io::Read;
use std::net::TcpListener;
use crate::http::Request;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.addr).expect("Failed to bind server port");
        println!("Simple server is running on {}", self.addr);

        loop {
            match listener.accept() {
                Ok((mut stream, client_address)) => {
                    println!("Client {} connected", client_address);
                    let mut buffer = [0; 1024];
                    stream.read(&mut buffer).expect("Failed to read message");
                    println!("Plain message received: {}", String::from_utf8_lossy(&buffer));
                    match Request::try_from(&buffer[..]) {
                        Ok(request) => {
                            dbg!(request);
                        },
                        Err(error) => println!("Failed to parse request: {:?}", error)
                    }
                },
                Err(e) => println!("Failed to establish a new connection: {}", e),
            }
        }
    }
}