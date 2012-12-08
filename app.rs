use std::*;
use io::WriterUtil;

use method::*;
use status::*;
use request::*;
use response::*;

struct Context {
    request: Request,
    response: Response
}

struct Handler {
    method: Method,
    pattern: ~str,
    callback: ~fn(&Context)
}

struct Application {
    mut handlers: ~[Handler]
}

impl Application {
    fn get(patt: &str, cb: ~fn(&Context)) {
        let h = Handler {
            method: method::GET,
            pattern: str::from_slice(patt),
            callback: move cb
        };
        self.handlers.push(move h);
    }
}

const NO_HANDLER_RESPONSE : &static/str = "
  <!DOCTYPE html>
  <html>
    <head>
      <title>Not found</title>
    </head>
    <body>
      There is no handler for that method and path
    </body>
  </html>";

fn run(f: fn(&Application)) {
    let app = Application {
        handlers: ~[]
    };
    f(&app);

    for fcgi::each_request() |req| {
        let ctx = Context {
            request: Request {
                method: from_str::from_str(req.get_param("REQUEST_METHOD")).get(),
                uri: req.get_param("REQUEST_URI")
            },
            response: Response {
                raw_request: req,
                status: OK,
                headers: map::HashMap(),
                headers_written: false
            }
        };

        let mut handled = false;

        for app.handlers.each() |h| {
            if ctx.request.method == h.method && str::starts_with(ctx.request.uri, h.pattern) {
                h.callback(&ctx);
                handled = true;
                break;
            }
        }

        if !handled {
            ctx.response.set_status(NOT_FOUND);
            ctx.response.set_header("Content-Type", "text/html");
            ctx.response.write_str(NO_HANDLER_RESPONSE);
        }

        ctx.response.write_headers();
    };
}

