use std::to_str::ToStr;

pub enum Status {
   	OK = 200,
    NOT_FOUND = 404
}

impl Status : ToStr {
    fn to_str(&self) -> ~str {
        match &self {
            OK => ~"OK",
            NOT_FOUND => ~"Not found"
        }
    }
}


