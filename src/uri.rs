
pub struct Uri {
    pub path: &'static str,
    pub query_string: &'static str,
}

impl Uri {
    pub fn new(path: &'static str, query_string: &'static str) -> Uri {
        Uri {
            path: path,
            query_string: query_string
        }
    }

}
