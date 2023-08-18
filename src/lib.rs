// use crate::page;
use hyper::{Body, Request, Response};
use std::convert::Infallible;
mod page;

// fn handle handles requests and return responses.
//
pub async fn handle(request: Request<Body>, page_path: &str) -> Result<Response<Body>, Infallible> {
    // pub async fn handle(request: Request<Body>, page_path: &str) {
    // ~/projects/wc/wc_html/src/page/mod.rs
    // pub struct Page page.source

    monitor_req_info(&request);

    // root page path + path requested
    let page_path = page_path.to_string() + request.uri().path();
    println!("page_path: {}", page_path);

    let mut page = match page::Page::from(&page_path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}: {:?}", page_path, e.kind());
            return res_404();
        }
    };

    if request.method() == hyper::Method::GET {
        return handle_get(&mut page);
    }

    if request.method() == hyper::Method::POST {
        return handle_post(&mut page, &request);
    }

    Ok(Response::new("Hello, world".into()))
}

fn handle_get(page: &mut page::Page) -> Result<Response<Body>, Infallible> {
    match page.body() {
        Ok(b) => return Ok(Response::new(b.into())),
        Err(_) => return res_404(),
    }
}

fn handle_post(
    page: &mut page::Page,
    request: &Request<Body>,
) -> Result<Response<Body>, Infallible> {
    // request
    // headers

    let wc_request = match request.headers().get("wc-request") {
        Some(ov) => match ov.to_str() {
            Ok(v) => v,
            Err(_) => return post_err_response(""),
        },
        None => return post_err_response(""),
    };

    if wc_request == "json_save" {
        return json_save(wc_request);
    }

    // Ok(Response::new("handle_post".into()))
    Ok(Response::new(
        r#"{"res":"post_handle page_json_save"}"#.into(),
    ))
    // Ok(r#"{"res":"post_handle page_json_save"}"#.to_string().into_bytes())
}

fn post_err_response(wc_request: &str) -> Result<Response<Body>, Infallible> {
    let msg = format!(
        r#"{{"res":"post_handle wc-request not found: {}"}}"#,
        wc_request
    );

    Ok(Response::new(msg.into()))
}

fn res_404() -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Not found".into()))
}

fn json_save(wc_request: &str) -> Result<Response<Body>, Infallible> {
    let msg = format!(r#"{{"res":"post_handle wc-request: {}"}}"#, wc_request);

    Ok(Response::new(msg.into()))
}

fn monitor_req_info(request: &Request<Body>) {
    // path
    println!("== request infor");
    println!("path: {}", request.uri().path());
    // path, query

    // headers
    let header_key = "host";
    if let Some(ov) = request.headers().get(header_key) {
        if let Ok(v) = ov.to_str() {
            println!("{}: {}", header_key, v);
        }
    }

    if request.method() == hyper::Method::GET {
        println!("method: GET");
    }
    if request.method() == hyper::Method::POST {
        println!("method: POST");
    }
}

// page data update
// Web -- Server
// file data to client
// wasm make html form data
