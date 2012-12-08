extern mod web;

use io::WriterUtil;
use web::*;

fn main() {
    do app::run() |app| {
        do app.get("/api/v1/test") |ctx| {
            ctx.response.set_status(status::OK);
            ctx.response.set_header("Content-Type", "application/json");
            ctx.response.write_str(~"{\"uri\":\"" + ctx.request.uri + "\"}");
        }
    }
}

