// server module
use std::net::TcpListener;

pub struct Server {
    address: String
}

impl Server {
    pub fn new(address: String) -> Self {
        Server {
            address
        }
    }

    pub fn run(self) { // takes ownership, deallocates Server when terminated
        println!("Listening on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap(); // ::bind returns Result<T, E> which can be (Ok(T), Err(E))
    }
}
