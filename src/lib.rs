use hyper::{Body, Request, Response};
use std::convert::Infallible;

// fn handle handles requests and return responses.
//
pub async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match header(&req, "host") {
        Ok(v) => println!("host: {}", v),
        Err(_) => (),
    }

    if req.method() == hyper::Method::GET {
        // println!("method: GET");
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

fn header<'a>(req: &'a Request<Body>, key: &str) -> Result<&'a str, ()> {
    let headers = req.headers();
    match headers.get(key) {
        Some(hv) => match hv.to_str() {
            Ok(v) => Ok(v),
            Err(_) => Err(()),
        },
        None => Err(()),
    }
}

// page data update
// Web -- Server
// file data to client
// wasm make html form data
