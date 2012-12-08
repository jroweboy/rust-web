use libc::*;
use io::WriterUtil;

extern mod core;
//pub enum Stream {}

pub type Request = {
	request_id: c_int,
	role: c_int,
	in_stream: *c_void,//*Stream,
	out_stream: *c_void,//*Stream,
	err_stream: *c_void,//*Stream,
	envp: **c_char,
	params_ptr: *c_void,
	ipc_fd: c_int,
	is_begin_processed: c_int,
	keep_connection: c_int,
	app_status: c_int,
	nwriters: c_int,
	flags: c_int,
	listen_sock: c_int
};

#[link_name="fcgi"]
extern mod lib {
fn FCGX_Init() -> c_int;
fn FCGX_InitRequest(req: *Request, sock: c_int, flags: c_int) -> c_int;
fn FCGX_Accept_r(req: *Request) -> c_int;
fn FCGX_GetParam(name: *c_char, envp: **c_char) -> *c_char;
fn FCGX_GetStr(s: *c_char, n: c_int, stream: *c_void) -> c_int;
fn FCGX_GetLine(s: *c_char, n: c_int, stream: *c_void) -> *c_char;
fn FCGX_PutStr(s: *c_char, n: c_int, stream: *c_void) -> c_int;
}

use lib::*;

pub fn each_request(f : fn(&Request) -> bool) -> bool {
	// there's no way to tell rust to default initialize
	// nor a way to say FCGX_Accept_r initializes req,
	// so this avoids a compiler error
	let req : Request = {
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
	if FCGX_Init() != 0 {
		io::stderr().write_line("FCGX_Init failed");
		return false
	}
	if FCGX_InitRequest(ptr::addr_of(&req), 0, 0) != 0 {
		io::stderr().write_line("FCGX_InitRequest failed");
		return false
	}
	while FCGX_Accept_r(ptr::addr_of(&req)) == 0 {
		if !f(&req) {
			break;
		}
	}
	true
}

pub fn get_param(req: &Request, name: &str) -> ~str {
	let res = do str::as_c_str(name) |cname| {
		FCGX_GetParam(cname, req.envp)
	} as *u8;
	if ptr::is_null(res) {
		return ~"";
	}
	unsafe {
		str::raw::from_buf(res)
	}
}

fn make_string(maxlen: uint, f : fn(*u8) -> uint) -> ~str {
	let mut result = str::with_capacity(maxlen);
	let mut length = 0u;
	do str::as_buf(result) |buf, _| {
		length = f(buf);
	}
	unsafe {
		str::raw::set_len(&mut result, length);
	}
	move(result)
}

pub fn get_string(req: &Request, maxlen: uint) -> ~str {
	do make_string(maxlen) |buf| {
		FCGX_GetStr(buf as *c_char, maxlen as c_int, req.in_stream) as uint
	}
}

pub fn get_line(req: &Request, maxlen: uint) -> ~str {
	do make_string(maxlen) |buf| {
		FCGX_GetLine(buf as *c_char, maxlen as c_int, req.in_stream);
		unsafe {
			do vec::raw::buf_as_slice(buf, maxlen) |v| {
				vec::position_elem(v, ~0).get()
			}
		}
	}
}

pub fn put_string(req: &Request, s: &str) {
	do str::as_buf(s) |buf, len| {
		FCGX_PutStr(buf as *c_char, len as c_int, req.out_stream);
	}
}

pub fn put_err_string(req: &Request, s: &str) {
	do str::as_buf(s) |buf, len| {
		FCGX_PutStr(buf as *c_char, len as c_int, req.err_stream);
	}
}
