use to_str::ToStr;

pub enum Status {
    OK = 200,
    NOT_FOUND = 404
}

impl Status : ToStr {
    pure fn to_str() -> ~str {
        match self {
            OK => ~"OK",
            NOT_FOUND => ~"Not found"
        }
    }
}


