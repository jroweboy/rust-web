use from_str::FromStr;

#[deriving_eq]
pub enum Method {
    OPTIONS,
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    TRACE
}

impl Method : FromStr {
    static pure fn from_str(s: &str) -> Option<Method> {
        if s == "OPTIONS" { Some(OPTIONS) }
        else if s == "GET" { Some(GET) }
        else if s == "HEAD" { Some(HEAD) }
        else if s == "POST" { Some(POST) }
        else if s == "PUT" { Some(PUT) }
        else if s == "DELETE" { Some(DELETE) }
        else if s == "TRACE" { Some(TRACE) }
        else { None }
    }
}

