// use crate::page;
use hyper::{Body, Request, Response};
use std::convert::Infallible;
mod page;

// fn handle handles requests and return responses.
//
pub async fn handle(req: Request<Body>, page_path: &str) -> Result<Response<Body>, Infallible> {
    // pub async fn handle(req: Request<Body>, page_path: &str) {
    // ~/projects/wc/wc_html/src/page/mod.rs
    // pub struct Page page.source

    monitor_req_info(&req);

    // root page path + path requested
    let page_path = page_path.to_string() + req.uri().path();
    println!("page_path: {}", page_path);

    let mut page = match page::Page::from(&page_path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}: {:?}", page_path, e.kind());
            return res_404();
        }
    };

    if req.method() == hyper::Method::GET {
        return handle_get(&mut page);
    }

    if req.method() == hyper::Method::POST {
        return handle_post();
    }

    Ok(Response::new("Hello, world".into()))
}

fn handle_get(page: &mut page::Page) -> Result<Response<Body>, Infallible> {
    match page.body() {
        Ok(b) => return Ok(Response::new(b.into())),
        Err(_) => return res_404(),
    }
}

fn handle_post() -> Result<Response<Body>, Infallible> {
    Ok(Response::new("handle_post".into()))
}

fn res_404() -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Not found".into()))
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
