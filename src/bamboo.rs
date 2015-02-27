



use hyper::server::Handler


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
    
    // before handle part
    fn before (&self, &mut Request);
    

    // main handle part
    fn handle (&self, &mut Request, &mut Response);


    // after handle part
    fn after (&self, &mut Response);
    
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

    }

    pub fn https (self, ip: IpAddr, port: Port, certificate: Path, key: path) -> HttpResult<listening> {
        
    }

    pub fn listen_with (mut self, ip: IpAddr, port: Port, nthreads: u32, protocol: Protocol) -> HttpResult<Listening> {

    }


}

// implement the hyper's Handler for Bamboo, intro the workflow entry
impl <H> Handler for Bamboo<H> where H: BambooHandler {
    
    //*** framework entry ***
    fn handle(&self, request: HyperRequest, response: HyperResponse<Fresh>) {
        // here we need convert HyperRequest to BambooRequest
        let mut req = ...;
        let mut res = ...;
        
        self.handler.before(&mut req);
        self.handler.handle(&mut req, &mut res);
        self.handler.after(&mut res);

        // and then, write response data back to client
        // TODO


    }


}

