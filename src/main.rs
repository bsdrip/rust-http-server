fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(); //never returns
}

struct Server {
    address: String,
}

impl Server {
    fn new(address: String) -> Self {
        Server {
            address
        }
    }

    fn run(self) { // takes ownership, deallocates Server when terminated
        println!("Listening on {}:", self.address);
    }
}


/* 
 * request
 * GET /user?id=10 HTTP/1.1\r\n
 * HEADERS \r\n
 * BODY
 */

struct Request {
    path: String,
    query_string: String,
    method: Method
}

enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}
