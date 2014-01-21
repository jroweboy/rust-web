use std::io;
use std::io::Stream;
use std::io::Writer;
use std::libc::types::common::c95::c_void;
use std::vec;
use std::ptr;
use std::c_str::CString;
use std::libc::types::os::arch::c95::c_char;
use fcgi::lib::{FCGX_GetParam, FCGX_GetStr, FCGX_GetLine, FCGX_PutStr};

pub struct FCGIRequest {
    request_id: i32,
    role: i32,
    in_stream: *Stream,
    out_stream: *Stream,
    err_stream: *Stream,
    envp: **i8,
    params_ptr: *c_void,
    ipc_fd: i32,
    is_begin_processed: i32,
    keep_connection: i32,
    app_status: i32,
    nwriters: i32,
    flags: i32,
    listen_sock: i32
}

impl FCGIRequest {
    fn get_param(&self, name: &str) -> ~str {
        let res = do name.to_c_str() |cname| {
            FCGX_GetParam(cname, self.envp)
        } as *c_char;

        let cstr : CString = CString::new(res);
        cstr.as_str()
    }

    fn get_string(&self, maxlen: uint) -> ~str {
        let mut buf = vec::with_capacity(maxlen as uint);
        //do make_string(maxlen) |buf| {
        FCGX_GetStr(buf as *i8, maxlen as i32, self.in_stream) as uint
        //}
    }

    fn get_line(&self, maxlen: uint) -> ~str {
        let mut buf = vec::with_capacity(maxlen as uint);
        //do make_string(maxlen) |buf| {
        FCGX_GetLine(buf as *i8, maxlen as i32, self.in_stream);
        unsafe {
            do vec::raw::buf_as_slice(buf, maxlen) |v| {
                v.position_elem(~0).get()
            }
        }
        //}
    }

    fn put_string(&self, s: &str) {
        do s.as_buf() |buf, len| {
            FCGX_PutStr(buf as *i8, (len - 1) as i32, self.out_stream);
        }
    }

    fn put_err_string(&self, s: &str) {
        do s.as_buf() |buf, len| {
            FCGX_PutStr(buf as *i8, (len - 1) as i32, self.err_stream);
        }
    }

    fn put_buf(&self, buf: &[u8]) {
        unsafe {
            //let ptr = to_const_ptr(buf);
            FCGX_PutStr(buf.as_ptr() as *i8, buf.len() as i32, self.out_stream);
        }
    }
}