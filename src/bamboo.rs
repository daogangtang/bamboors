
use std::net::{SocketAddr, IpAddr};
use std::os;
use std::path::Path;

use hyper::server::{Handler, Server, Listening};
use hyper::server::request::Request as HttpRequest;
use hyper::server::response::Response as HttpResponse;
use hyper::net::Fresh;
use hyper::{HttpResult, HttpError};

use request::Request;
use response::Response;
use error::BambooError;

#[derive(Clone)]
pub enum Protocol {
    Http,
    Https {
        // Path to SSL certificate file
        certificate: Box<Path>,
        // Path to SSL private key file
        key: Box<Path>
    }

}

// TODO: check the type here, now is String
//pub type BambooResult<String> = Result<String, BambooError>;
pub type BambooResult<T> = Result<T, BambooError>;


// other handler entities should implement this
pub trait BambooHandler: Send + Sync + 'static {
    
    // main handle part
    fn handle (&self, &mut Request, &mut Response) -> BambooResult<String>;
    
}

pub struct Bamboo<H: BambooHandler> {
    pub handler: H,

}

impl <H: BambooHandler> Bamboo<H> {
    pub fn new(handler: H) -> Bamboo<H> {
        Bamboo {
            handler: handler,
        }
    }

    pub fn http (self, ip: IpAddr, port: u16) -> HttpResult<Listening> {
        self.listen_with(ip, port, os::num_cpus() * 5 / 4, Protocol::Http)
    }

    pub fn https (self, ip: IpAddr, port: u16, certificate: Path, key: Path) -> HttpResult<Listening> {
        self.listen_with(ip, port, os::num_cpus() * 5 / 4, Protocol::Https {
            certificate: certificate, 
            key: key
        }) 
    }

    pub fn listen_with (mut self, ip: IpAddr, port: u16, nthreads: usize, protocol: Protocol) -> HttpResult<Listening> {
        let server = match protocol {
            Protocol::Http 
                => Server::http(self),
            Protocol::Https { ref certificate, ref key } 
                => Server::https(self, certificate.clone(), key.clone())
        };
        
        Ok(try!(server.listen_threads(ip, port, nthreads)))
    }


}

// implement the hyper's Handler for Bamboo, intro the workflow entry
impl<H> Handler for Bamboo<H> where H: BambooHandler {
    
    //*** framework entry ***
    fn handle(&self, request: HttpRequest, response: Response<Fresh>) {
        // here we need convert HyperRequest to BambooRequest
        let mut req = Request::new(request);
        let mut res = response;
       
        // logic into
        let ret = self.handler.handle(&mut req, &mut res);

        // and then, write response data back to client
        match ret {
            Ok(ref ret) => {
                res.write2client();
                res.start();
                //res.write_all(ret);
                // for test
                res.write_all(b"hello bamboo.");
                res.end();
                return;
            },
            Err(e) => {
                println!("[Error] Error: {:?}\nRequest: {:?}\n", e, req);
            }
        }


    }


}

// implement BambooHandler for function handler kind, so function can
// call its self by:  handle_func.handle(req, res)
impl<F> BambooHandler for F 
    where F: Send + Sync + 'static + Fn(&mut Request, &mut Response) {
    
    fn handle(&self, req: &mut Request, res: &mut Response) -> BambooResult<String> {
       (*self)(req, res);
    }

}

// syntax sugar
impl BambooHandler for Box<BambooHandler> {
    
    fn handle(&self, req: &mut Request, res: &mut Response) -> BambooResult<String> {
        (**self).handle(req);
    }
}


