// server module
use std::net::TcpListener;
use std::io::Read;

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

        loop {
            match listener.accept() { // hangs until new connection arrives, returns a Result<(TcpStream, SocketAddr)> because might fail
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) { // reads bytes from socket
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                        },
                        Err(e) => println!("Failed to read from connection: {}", e)
                    }
                },

                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
