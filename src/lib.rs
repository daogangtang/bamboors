#![feature(unboxed_closures, core, os, io, net, path)]


extern crate hyper;
extern crate url;
extern crate "route-recognizer" as recognizer;


pub use bamboo::{Bamboo, BambooHandler, Protocol};
pub use error::BambooError;
pub use request::Request;
pub use response::Response;
pub use typemap::TypeMap;
pub use router::Router;

pub mod prelude {
    pub use {
        Bamboo,
        BambooHandler,
        Protocol,
        BambooError,
        Request,
        Response,
        TypeMap,
        Router


    };

}


mod bamboo;
mod error;
mod request;
mod response;
mod uri;
mod typemap;
mod router;



#[test]
fn it_works() {
}
