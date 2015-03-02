
pub struct Uri {
    path: &str,
    query_string: &str,
}

impl Uri {
    pub fn new(path: &str, query_string: &str) -> Uri {
        Uri {
            path: path,
            query_string: query_string
        }
    }

}
