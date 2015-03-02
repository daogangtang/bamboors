
use std::net::ip::SocketAddr;
use std::os;

use hyper::server::{Handler, Server, Listening};
use hyper::server::request::Request as HttpRequest;
use hyper::server::response::Response as HttpResponse;
use hyper::net::Fresh;
use hyper::{HttpResult, HttpError};

pub enum Protocol {
    Http,
    Https {
        // Path to SSL certificate file
        certificate: Path,
        // Path to SSL private key file
        key: Path
    }

}

// other handler entities should implement this
pub trait BambooHandler: Send + Sync {
    
    // main handle part
    fn handle (&self, &mut Request, &mut Response);
    
}

pub struct Bamboo<H: BambooHandler> {
    pub handler: H,

    pub addr: Option<SocketAddr>,

    pub protocol: Option<Protocol>

}

impl <H: BambooHandler> Bamboo<H> {
    pub fn new(handler: H) -> Bamboo<H> {
        Bamboo {
            handler: handler,
            addr: None,
            protocol: None
        }
    }

    pub fn http (self, ip: IpAddr, port: Port) -> HttpResult<Listening> {
        self.listen_with(ip, port, os::num_cpus() * 5 / 4, Protocol::Http)
    }

    pub fn https (self, ip: IpAddr, port: Port, certificate: Path, key: path) -> HttpResult<listening> {
        self.listening(ip, port, os::num_cpus() * 5 / 4, Protocol::Https {
            certificate: certificate, 
            key: key
        }) 
    }

    pub fn listen_with (mut self, ip: IpAddr, port: Port, nthreads: usize, protocol: Protocol) -> HttpResult<Listening> {
        let server = match protocol {
            Protocol::Http 
                => Server::http(ip, port),
            Protocol::Https { ref certificate, ref key } 
                => Server::https(ip, port, certificate.clone(), key.clone())
        };
        
        self.protocol = Some(protocol);

        Ok(try!(server.listen_threads(self, nthreads)))
    }


}

// implement the hyper's Handler for Bamboo, intro the workflow entry
impl <H> Handler for Bamboo<H> where H: BambooHandler {
    
    //*** framework entry ***
    fn handle(&self, request: HttpRequest, response: HttpResponse<Fresh>) {
        // here we need convert HyperRequest to BambooRequest
        let mut req = Request::new(request);
        let mut res = response;
        
        self.handler.before(&mut req);
        self.handler.handle(&mut req, &mut res);
        self.handler.after(&mut res);

        // and then, write response data back to client
        // TODO


    }


}

