use std::io;
use std::io::Writer;
use std::libc::types::common::c95::c_void;
use std::vec;
use std::ptr;
use fcgirequest::FCGIRequest;

// for whatever reason I can't call to_const_ptr in libcore (but I can call to_ptr)
//#[inline(always)]
//pub unsafe fn to_const_ptr<T>(v: &[T]) -> *T {
//    let repr: **vec::raw::SliceRepr = ::cast::transmute(&v);
//    return ::cast::reinterpret_cast(&ptr::addr_of(&((**repr).data)));
//}

#[link_name="fcgi"]
mod lib {
    use fcgirequest::FCGIRequest;
    //use std::io::Stream;
    enum Stream {}
    extern {
        fn FCGX_Init() -> i32;
        fn FCGX_InitRequest(req: *FCGIRequest, sock: i32, flags: i32) -> i32;
        fn FCGX_Accept_r(req: *FCGIRequest) -> i32;
        fn FCGX_GetParam(name: *i8, envp: **i8) -> *i8;
        fn FCGX_GetStr(s: *i8, n: i32, stream: *Stream) -> i32;
        fn FCGX_GetLine(s: *i8, n: i32, stream: *Stream) -> *i8;
        fn FCGX_PutStr(s: *i8, n: i32, stream: *Stream) -> i32;
    }
}


pub fn each_request(f : fn(&FCGIRequest) -> bool) -> bool {
    // there's no way to tell rust to default initialize
    // nor a way to say FCGX_Accept_r initializes req,
    // so this avoids a compiler error
    let req = FCGIRequest {
        request_id: 0,
        role: 0,
        in_stream: ptr::null(),
        out_stream: ptr::null(),
        err_stream: ptr::null(),
        envp: ptr::null(),
        params_ptr: ptr::null(),
        ipc_fd: 0,
        is_begin_processed: 0,
        keep_connection: 0,
        app_status: 0,
        nwriters: 0,
        flags: 0,
        listen_sock: 0
    };
    if lib::FCGX_Init() != 0 {
        io::stderr().write_line("FCGX_Init failed");
        return false
    }
    if lib::FCGX_InitRequest(req.addr_of(), 0, 0) != 0 {
        io::stderr().write_line("FCGX_InitRequest failed");
        return false
    }
    while lib::FCGX_Accept_r(req.addr_of()) == 0 {
        if !f(&req) {
            break;
        }
    }
    true
}

/*
// DISABLED until I figure out what this is good for :p
fn make_string(maxlen: uint, f : fn(*u8) -> uint) -> ~str {
    let mut result = vec::with_capacity(maxlen);
    let mut length = 0u;
    do result.as_buf() |buf, _| {
        length = f(buf);
    }
    unsafe {
        str::raw::set_len(&mut result, length);
    }
    move(result)
}
*/