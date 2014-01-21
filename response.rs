use status::Status;
use fcgirequest::FCGIRequest;
use std::hashmap::HashMap;
use std::io;

pub struct Response {
    raw_request: &FCGIRequest,
    status : Status,
    headers : HashMap<&str, &str>,
    headers_written : bool,
}

impl Response {
    fn set_status(&self, status: Status) {
        self.status = status;
    }
    fn set_header(&self, name: &str, value: &str) {
        self.headers.insert(name.to_managed(), value.to_managed());
    }
    fn write_headers(&self) {
        if self.headers_written { return }
        let mut headers = ~"Status: " + (self.status as uint).to_str() +
            " " + self.status.to_str() + "\n";
        for (name, value) in self.headers.each_ref() {
            headers = headers + *name + ": " + *value + "\n";
        }
        headers = headers + "\n";
        self.raw_request.put_string( headers );
        self.headers_written = true;
    }
}

impl Response : io::Writer {
    fn write(&self, v: &[u8]) {
        self.write_headers();
        self.raw_request.put_buf(v);
    }
    fn seek(&self, _: int, _: io::SeekStyle) {}
    fn tell(&self) -> uint { 
        0u 
    }
    fn flush(&self) -> int { 
        0 
    }
    fn get_type(&self) -> io::Writer { 
        // TODO: Figure out what I'm supposed to return here
        //std::io::File 
    }
}

