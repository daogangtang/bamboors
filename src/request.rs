extern crate url;
use url::Url;
use url::form_urlencoded::parse as parse_query_string;

use hyper::server::request::Request as HttpRequest;
use hyper::uri::RequestUri::AbsolutePath;
use hyper::http::HttpReader;
use hyper::version::HttpVersion;
use hyper::method::Method;
use hyper::header::Headers;

use std::net::SocketAddr;
use std::io::Result as IoResult;

use std::collections::HashMap;
use bamboo::BambooResult;
use uri::Uri;
use typemap::TypeMap;

/// The body of an Iron request,
pub struct Body<'a>(HttpReader<&'a mut (Reader + 'a)>);

impl<'a> Body<'a> {
    pub fn new(reader: HttpReader<&'a mut (Reader + 'a)>) -> Body<'a> {
        Body(reader)
    }   
}

impl<'a> Reader for Body<'a> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        self.0.read(buf)
    }   
}



pub struct Request<'a> {
    
    pub version: HttpVersion,
    
    pub method: Method,

    pub uri: Uri, // path, query_string, etc...
    
    pub headers: Headers,

    pub remote_addr: SocketAddr,

    pub body: Body<'a>,

    pub params: HashMap<String, String>,  // params data
    
    pub meta: TypeMap,  // extension data

}

impl<'a> Request<'a> {
    pub fn new(request: HttpRequest) -> BambooResult<Request> {
        let (remote_addr, method, headers, orig_uri, version, body) = request.deconstruct();

        let mut params = HashMap::new();
        let mut uri: Uri;  // = Uri::new();
        match orig_uri {
            AbsolutePath(ref path) => {
                // TODO: here, we should do the checking of trailing '/'s, and normalize it as one trailing '/'
                //  
                
                let uri_vec: Vec<&str> = path.split('?').collect();
                //uri.path = uri_vec[0];
                //uri.query_string = uri_vec[1];
                uri = Uri::new(uri_vec[0], uri_vec[1]);

                let query_vec = parse_query_string(uri.query_string.as_bytes());

                // path.split('?')
                // use  url::form_urlencoded::parse to parse query_string 
                // TODO: optimaze this
                // because url module can only parse full url form, not relative url form, here we construct a full url first
                //let url = format!("http://localhost/{}", path);
                // get the query part as a vec
                //let query_vec = match Url::parse(url.as_slice()) {
                //    Ok(url) => url.query_pairs().unwrap(),
                //    Err(e)  => return Err(format!("Couldn't parse requested URL: {}", e))
                //};
                // transform it to hashmap, store them in params
                for elem in query_vec  {
                    params.insert(elem.0, elem.1);
                }
            },
            _ => return Err("unsupported request URI".to_string())
        };

        Ok(Request {
            version: version,
            method: method,
            remote_addr: remote_addr,
            headers: headers,
            // path ...
            uri: uri,
            body: Body::new(body),
            params: params,
            meta: TypeMap::new(),
        })

    }

}

