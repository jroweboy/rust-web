use std::*;
use status::Status;

pub struct Response {
    raw_request: &fcgi::Request,
    mut status : Status,
    headers : map::HashMap<@str, @str>,
    mut headers_written : bool,
}

impl Response {
    fn set_status(status: Status) {
        self.status = status;
    }
    fn set_header(name: &str, value: &str) {
        self.headers.insert(name.to_managed(), value.to_managed());
    }
    fn write_headers() {
        if self.headers_written { return }
        let mut headers = ~"Status: " + (self.status as uint).to_str() +
            " " + self.status.to_str() + "\n";
        for self.headers.each_ref() |name, value| {
            headers = headers + *name + ": " + *value + "\n";
        }
        headers = headers + "\n";
        self.raw_request.put_string( headers );
        self.headers_written = true;
    }
}

impl Response : io::Writer {
    fn write(v: &[const u8]) {
        self.write_headers();
        self.raw_request.put_buf(v);
    }
    fn seek(_: int, _: io::SeekStyle) {}
    fn tell() -> uint { 0u }
    fn flush() -> int { 0 }
    fn get_type() -> io::WriterType { io::File }
}

