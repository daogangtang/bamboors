


pub struct Request {
    
    pub version: HttpVersion,
    
    pub method: Method,

    pub uri: Uri, // path, query_string, etc...
    
    pub headers: Headers,

    pub remote_addr: SocketAddr,

    pub body: Body,

    pub params: HashMap<String, String>,  // params data
    
    pub meta: TypeMap,  // extension data

}

impl Request {
    
    pub fn from_hyper(req: HyperRequest) -> Option<Request> {

    }


}
