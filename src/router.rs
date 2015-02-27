



use recognizer::Router as Recognizer;
use recognizer::{Match, Params};

pub struct Router {
    router_builder:  Recognizer<Box<BambooHandler>>
}

impl Router {
   
    // constructor
    pub fn new() -> Router {
        router_builder: Recognizer::new()
    }

    // use this method to add url pattern
    fn add<H: BambooHandler> (&mut self, pattern: &str, handler: H)
        -> &mut Router {


        self
    }

    // this method to recognize the path by previously added patterns
    fn recognize(&self, path: &str) -> Option<Match<Box<BambooHandler>>> {


    }


}
