use std::io::Writer;
use response::Response;
use request::Request;
use method::Method;

struct Context {
    request: Request,
    response: Response
}

struct Handler {
    method: Method,
    pattern: ~str,
    callback: ~|&Context|
}

struct Application {
    pub handlers: ~[Handler]
}

impl Application {
    fn get(patt: &str, cb: ~|&Context|) {
        let h = Handler {
            method: Some(Method::GET),
            pattern: str::from_slice(patt),
            callback: cb
        };
        self.handlers.push(h);
    }
}

static NO_HANDLER_RESPONSE : &'static str = "
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

    for req in fcgi::each_request() {
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

        for h in app.handlers.each() {
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

