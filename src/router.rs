
use bamboo::{BambooHandler, BambooResult};
use error::BambooError;
use request::Request;
use response::Response;

use recognizer::Router as Recognizer;
use recognizer::{Match, Params};

pub struct Router {
    router_builder:  Recognizer<Box<BambooHandler>>,
    before_handle: Box<Fn(&mut Request) -> BambooResult<String>>,
    after_handle: Box<Fn(&mut Response) -> BambooResult<String>>,
}

impl Router {
   
    // constructor
    pub fn new() -> Router {
        Router {
            router_builder: Recognizer::new(),
            before_handle: Box::new(|_: &mut Request|->BambooResult<String>{ Ok("".to_string())}),
            after_handle: Box::new(|_: &mut Response|->BambooResult<String>{ Ok("".to_string())})
        }
    }
    
    pub fn new_with_middleware(before_handle: Box<Fn(&mut Request)->BambooResult<String>>, after_handle: Box<Fn(&mut Response)->BambooResult<String>>) -> Router {
        Router {
            router_builder: Recognizer::new(),
            before_handle: before_handle,
            after_handle: after_handle
        }
    }

    // use this method to add url pattern
    pub fn add<H: BambooHandler> (&mut self, pattern: &str, handler: H)
        -> &mut Router {
        
        self.router_builder.add(pattern, Box::new(handler) as Box<BambooHandler>);
        // return self to trailing style expression
        self
    }

    // this method to recognize the path by previously added patterns
    fn recognize<'a>(&'a self, path: &str) -> Option<Match<&'a Box<BambooHandler>>> {
        self.router_builder.recognize(path).ok()
    }
   
    // here, Request is Bamboo Request
    fn execute(&self, path: &str, req: &mut Request, res: &mut Response) -> BambooResult<String> {
        let matched = match self.recognize(path) {
            Some(matched) => matched,

            // No match
            None => return Err(BambooError::new("none"))
        };

        // here, we need to extract matched.params and dump them into req.params
        for (k, v) in matched.params.map.iter() {
            req.params.insert(k.clone(), v.clone());
        }

        // execute the truely function handler
        // corresponding to hyper
        matched.handler.handle(req, res)

    }

}

unsafe impl Send for Router {}
unsafe impl Sync for Router {}

// implement this, make Router become a acceptable handler
impl BambooHandler for Router {
    
    fn handle(&self, req: &mut Request, res: &mut Response) -> BambooResult<String> {
        // before from Middleware trait
        let mut ret = (self.before_handle)(req);
        match ret {
            Ok(_) => {
                // main execution
                let path = req.uri.path;
                ret = self.execute(path, req, res);
                match ret {
                    Ok(body) => {
                        // after from Middleware trait
                        (self.after_handle)(res).unwrap();
                        // once handler produce body, post middleware couldn't modify it?
                        Ok(body)
    
                    },
                    Err(e) => Err(e)
                }
            },
            Err(e) => Err(e)
        }
    }

}

