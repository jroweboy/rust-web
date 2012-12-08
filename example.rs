extern mod web;

use web::*;

fn main() {
    for fcgi::each_request() |req| {
        let method = fcgi::get_param(req, "REQUEST_METHOD");
        let uri = fcgi::get_param(req, "REQUEST_URI");

        let mut headers = ~"Status: 200 OK\n";
        str::push_str(&mut headers, "Content-Type: text/html\n");
        str::push_str(&mut headers, "\n\n");
        fcgi::put_string(req, headers);

        fcgi::put_string(req, "<!DOCTYPE html>\n");
        fcgi::put_string(req, "<html><head></head><body>\n");
        fcgi::put_string(req, "Hello, world!\n");
        fcgi::put_string(req, ~"Method: " + method + "\n");
        fcgi::put_string(req, ~"URI: " + uri + "\n");
        fcgi::put_string(req, "</body></html>\n");
    };
}
