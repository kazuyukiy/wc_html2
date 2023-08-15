use hyper::{Body, Request, Response};
use std::convert::Infallible;

// fn handle handles requests and return responses.
//
pub async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // ~/projects/wc/wc_html/src/page/mod.rs
    // pub struct Page page.source

    monitor_req_info(&req);

    if req.method() == hyper::Method::GET {
        return handle_get();
    }

    if req.method() == hyper::Method::POST {
        // println!("method: POST");
        return handle_post();
    }

    Ok(Response::new("Hello, world".into()))
}

fn handle_get() -> Result<Response<Body>, Infallible> {
    Ok(Response::new("handle_get".into()))
}

fn handle_post() -> Result<Response<Body>, Infallible> {
    Ok(Response::new("handle_post".into()))
}

fn monitor_req_info(req: &Request<Body>) {
    // path
    println!("== req infor");
    println!("path: {}", req.uri().path());
    // path, query

    // headers
    let header_key = "host";
    if let Some(ov) = req.headers().get(header_key) {
        if let Ok(v) = ov.to_str() {
            println!("{}: {}", header_key, v);
        }
    }

    if req.method() == hyper::Method::GET {
        println!("method: GET");
    }
    if req.method() == hyper::Method::POST {
        println!("method: POST");
    }
}

// page data update
// Web -- Server
// file data to client
// wasm make html form data
