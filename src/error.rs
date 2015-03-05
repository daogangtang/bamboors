

// import StdError trait;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub struct BambooError {
    desc: &'static str,
}

impl BambooError {
    pub fn new(desc: &'static str) {
        BambooError {
            desc: desc 
        }
    }
}

impl StdError for BambooError {
    fn description(&self) -> &str {
        self.desc
    }

    fn cause(&self) -> Option<&StdError> {

    }
}

impl fmt::Display for BambooError {
    fn fmt(&self, f: *mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self.desc, f)
    }
}
