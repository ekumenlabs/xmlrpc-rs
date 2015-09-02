use std::net::{ToSocketAddrs, SocketAddr};
use hyper::server::Server;
use hyper::net::Fresh;
use hyper::server::Listening;
use hyper::error::Result as HttpResult;
use hyper::server::request::Request as HttpRequest;
use hyper::server::response::Response as HttpResponse;
use std::io::Read;

pub struct XmlrpcServer {
    addr: Option<SocketAddr>
}

impl XmlrpcServer {
    /// Starts the http server with X number of threads
    ///
    /// ## Panics
    ///
    /// Panics if the provider address does not parse. To avoid this
    /// call `to_socket_addr` yourself and pass a parsed `SocketAddr`.
    pub fn serve<A: ToSocketAddrs>(mut self, addr: A, threads: usize) -> HttpResult<Listening> {
        let sock_addr = addr.to_socket_addrs()
            .ok().and_then(|mut addr| addr.next()).expect("Could not parse socket address.");

        self.addr = Some(sock_addr);
        try!(Server::http(sock_addr)).handle_threads(self, threads)
    }

    pub fn new() -> XmlrpcServer {
        XmlrpcServer { addr: None }
    }
}

impl ::hyper::server::Handler for XmlrpcServer {
    fn handle(&self, mut http_req: HttpRequest, http_res: HttpResponse<Fresh>) {
        let mut req_str = String::new();
        match http_req.read_to_string(&mut req_str) {
            Ok(_) => println!("Request: {}", req_str),
            Err(_) => panic!("Cannot convert to string.")
        }
    }
}

#[cfg(test)]
mod test {
    use super::XmlrpcServer;

    #[test]
    fn test_serve() {
        let xmlrpc = XmlrpcServer::new();
        xmlrpc.serve("localhost:3000", 2).unwrap();
    }
}
