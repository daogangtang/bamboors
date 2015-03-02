



use recognizer::Router as Recognizer;
use recognizer::{Match, Params};

pub struct Router {
    router_builder:  Recognizer<Box<BambooHandler>>
}

impl Router {
   
    // constructor
    pub fn new() -> Router {
        Router {
            router_builder: Recognizer::new()
        }
    }

    // use this method to add url pattern
    fn add<H: BambooHandler> (&mut self, pattern: &str, handler: H)
        -> &mut Router {
        
        self.router_builder.add(pattern, Box::new(handler) as Box<BambooHandler>)
        // return self to trailing style expression
        self
    }

    // this method to recognize the path by previously added patterns
    fn recognize(&self, path: &str) -> Option<Match<Box<BambooHandler>>> {
        self.router_builder.recognize(path).ok()
    }
   
    // here, Request is Bamboo Request
    fn execute(&self, path: &str, req: &mut Request, res: &mut Respose) -> Result<bool> {
        let matched = match self.recognize(path) {
            Some(matched) => matched,

            // No match
            None => return Err()
        };

        // here, we need to extract matched.params and dump them into req.params
        for (k, v) in matched.params.map.iter() {
            req.params.insert(k, v);
        }

        // execute the truely function handler
        matched.handler.call(req, res)

    }

}

// implement this, make Router become a acceptable handler
impl BambooHandler for Router {
    
    fn handle(&self, req: &mut Request, res: &mut Respose) -> Result<bool> {
        // before from Middleware trait
        self.before(req);
        
        // main execution
        let path = req.uri.path;
        self.execute(path, req, res);

        // after from Middleware trait
        self.after(res);

    }

}
