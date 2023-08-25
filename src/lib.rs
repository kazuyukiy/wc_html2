use hyper::{Body, Request, Response};
use std::convert::Infallible;
pub mod page;

// fn handle handles requests and return responses.
//
pub async fn handle(request: Request<Body>, page_path: &str) -> Result<Response<Body>, Infallible> {
    // root page path + path requested
    let page_path = page_path.to_string() + request.uri().path();
    println!("page_path: {}", page_path);

    let mut wc_handler = WcHandler::new(&request, &page_path);

    monitor_req_info(&request);

    wc_handler.handle()
    // remove return and ;
    // after all the rest after this line has beem moved to wc_handler.handle()
    // return wc_handler.handle();
    // Ok(Response::new("Hello, world".into()))
}

struct WcHandler<'a> {
    request: &'a Request<Body>,
    page_path: &'a str,
    page: Option<page::Page>,
    wc_request: Option<&'a str>,
}

impl<'a> WcHandler<'a> {
    fn new(request: &'a Request<Body>, page_path: &'a str) -> WcHandler<'a> {
        println!("page_path: {}", page_path);

        WcHandler {
            request,
            page_path,
            page: None,
            wc_request: None,
        }
    }

    fn handle(&mut self) -> Result<Response<Body>, Infallible> {
        match page::Page::from(self.page_path) {
            Ok(page) => self.page.replace(page),
            Err(e) => {
                eprintln!("{}: {:?}", self.page_path, e.kind());
                return res_404();
            }
        };

        if self.request.method() == hyper::Method::GET {
            return self.handle_get();
        }

        if self.request.method() == hyper::Method::POST {
            return self.handle_post();
        }
        // temp
        Ok(Response::new("Hello, world".into()))
    }

    fn handle_get(&mut self) -> Result<Response<Body>, Infallible> {
        match self.page.as_mut().unwrap().body() {
            Ok(b) => return Ok(Response::new(b.into())),
            Err(_) => return res_404(),
        }
    }

    fn handle_post(&mut self) -> Result<Response<Body>, Infallible> {
        // request
        // headers

        match self.request.headers().get("wc-request") {
            Some(ov) => match ov.to_str() {
                Ok(v) => self.wc_request.replace(v),
                Err(_) => return self.post_err_response(),
            },
            None => return self.post_err_response(),
        };

        let wc_request = self.wc_request.as_ref().unwrap();

        if wc_request == &"json_save" {
            return self.json_save();
        }

        Ok(Response::new(
            r#"{"res":"post_handle page_json_save"}"#.into(),
        ))
    }

    fn post_err_response(&self) -> Result<Response<Body>, Infallible> {
        let msg = format!(
            r#"{{"res":"post_handle wc-request not found: {}"}}"#,
            self.wc_request.as_ref().unwrap() // wc_request
        );

        Ok(Response::new(msg.into()))
    }
    fn json_save(&self) -> Result<Response<Body>, Infallible> {
        let msg = format!(
            r#"{{"res":"post_handle wc-request: {}"}}"#,
            self.wc_request.as_ref().unwrap()
        );

        Ok(Response::new(msg.into()))
    }
}

fn res_404() -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Not found".into()))
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
