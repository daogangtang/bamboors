extern crate url;
use url::Url;
use url::form_urlencoded::parse as parse_query_string;

use hyper::server::request::Request as HttpRequest;
use hyper::uri::RequestUri::AbsolutePath as AbsolutePath;
use hyper::uri::RequestUri::AbsoluteUri;
use hyper::http::HttpReader;
use hyper::version::HttpVersion;
use hyper::method::Method;
use hyper::header::Headers;

use std::net::SocketAddr;
use std::io::Result as IoResult;
use std::io::{Read, Write};
use std::fmt::{self, Debug};

use std::collections::HashMap;
use bamboo::BambooResult;
use error::BambooError;
use uri::Uri;
use typemap::TypeMap;

/// The body of an Iron request,
pub struct Body<'a>(HttpReader<&'a mut (Read + 'a)>);

impl<'a> Body<'a> {
    pub fn new(reader: HttpReader<&'a mut (Read + 'a)>) -> Body<'a> {
        Body(reader)
    }   
}

impl<'a> Read for Body<'a> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        self.0.read(buf)
    }   
}



pub struct Request<'a> {
    
    pub version: HttpVersion,
    
    pub method: Method,

    pub uri: Uri<'a>, // path, query_string, etc...
    
    pub headers: Headers,

    pub remote_addr: SocketAddr,

    pub body: Body<'a>,

    pub params: HashMap<String, String>,  // params data
    
    pub meta: TypeMap,  // extension data

}

impl<'a> Debug for Request<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       Ok(()) 
    }
}

impl<'a> Request<'a> {
    pub fn new(request: HttpRequest<'a>) -> BambooResult<Request<'a>> {
        let (remote_addr, method, headers, orig_uri, version, body) = request.deconstruct();
        let path = match orig_uri {
            AbsolutePath(ref _path) => {
                _path
            },
            _ => return Err(BambooError::new("Unsupported request URI"))
        };

        let uri_vec: Vec<&str> = path.split('?').collect();
        let uri: Uri;
        if uri_vec.len() == 1 {
            uri = Uri::new(uri_vec[0], "");
        }
        else {
            uri = Uri::new(uri_vec[0], uri_vec[1]);
        }

        let query_vec = parse_query_string(uri.query_string.as_bytes());

        let mut params = HashMap::new();
        // transform it to hashmap, store them in params
        for elem in query_vec  {
            params.insert(elem.0, elem.1);
        }
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

