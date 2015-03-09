
pub struct Uri<'a> {
    pub path: &'a str,
    pub query_string: &'a str,
}

impl<'a> Uri<'a> {
    pub fn new(path: &'a str, query_string: &'a str) -> Uri<'a> {
        Uri {
            path: path,
            query_string: query_string
        }
    }

}
