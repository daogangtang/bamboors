extern crate hyper;
extern crate bamboo;

use bamboo::prelude::*;
use hyper::status;
use std::io::Write;
use std::net::IpAddr;


fn main() {
    Bamboo::new(|_: &mut Request, _: &mut Response| -> BambooResult<String> {
        Ok("Hahaha".to_string())
    }).http(IpAddr::new_v4(127, 0, 0, 1), 3000).unwrap();
}
