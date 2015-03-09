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
        //let path = match orig_uri {
        let url_obj = match orig_uri {
            AbsolutePath(ref _path) => {
//                _path
                let url_str = format!("http://localhost/{}", _path);
                match Url::parse(url_str.as_slice()) {
                    Ok(url) => url,
                    Err(e) => return Err(BambooError::new("Couldn't parse requested URL: {}"))
                }
            },
            _ => return Err(BambooError::new("Unsupported request URI"))
        };

        //let path: String = format!("/{}", url_obj.path().unwrap().connect("/"));
        let path: String = "/".to_string();
        //let query_string: String = format!("{}", query_str_obj.to_string());
        //let query_string: String = "".to_string();
//        let uri: Uri = Uri::new(path.as_slice(), "");
        let uri: Uri = Uri::new("", "");
        //let query_str_obj = url_obj.query_pairs().unwrap();
        let query_str_obj = vec![("xxx".to_string(), "yyy".to_string())];

        //let tmp: &str = format!("{}", path).as_slice();

        //let deliter = path.find('?').unwrap();
        //let uri_vec: Vec<&str> = path.split('?').collect();
        //let uri_vec: Vec<&str> = path.split('?').collect();
        //let uri_vec: Vec<&str> = vec!["/", ""];
        //let uri_vec: Vec<&str> = vec![&path[..deliter], &path[(deliter+1)..]];
        //if uri_vec.len() == 1 {
        //    uri = Uri::new(uri_vec[0], "");
        //}
        //else {
        //    uri = Uri::new(uri_vec[0], uri_vec[1]);
            //let mut uri: Uri = Uri::new("/", "");
        //}

        //let query_vec = parse_query_string(uri.query_string.as_bytes());

        let mut params = HashMap::new();
        // transform it to hashmap, store them in params
        //for elem in query_vec  {
        for elem in query_str_obj  {
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
/*
        let mut uri: Uri = Uri::new("/", "");
        let mut params = HashMap::new();
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
*/

    }

}

