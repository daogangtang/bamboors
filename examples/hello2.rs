extern crate hyper;
extern crate bamboo;

use bamboo::prelude::*;
use hyper::status;
use std::io::Write;
use std::net::IpAddr;


fn main() {
    let mut router = Router::new();
    router.add("/9", hello9);
    router.add("/", hello);

   
    fn hello(req: &mut Request, res: &mut Response) -> BambooResult<String> {
        Ok("Hello tgg".to_string())
    }
    
    fn hello9(req: &mut Request, res: &mut Response) -> BambooResult<String> {
        Ok("Hello 999".to_string())
    }

    Bamboo::new(router).http(IpAddr::new_v4(127, 0, 0, 1), 3000).unwrap();
}
