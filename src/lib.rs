use hyper::body::HttpBody;
use hyper::header::HeaderValue;
use hyper::{Body, Method, Request, Response, StatusCode};

pub mod page;

pub async fn service(
    request: Request<Body>,
    source_path: &str,
) -> Result<Response<Body>, hyper::Error> {
    let mut handler = WcHandler::from(request, source_path);

    handler.handle().await

    // Ok(Response::new("Hello, world".into()))
}

struct WcHandler<'a> {
    source_path: &'a str,
    request: Request<Body>,
    page: Option<page::Page>,
}

impl<'a> WcHandler<'a> {
    fn from(request: Request<Body>, source_path: &'a str) -> WcHandler {
        let wc_handler = WcHandler {
            source_path,
            request: request,
            page: None,
        };

        wc_handler
    }

    fn wc_request(&self) -> Option<&HeaderValue> {
        self.request.headers().get("wc-request")
    }

    fn body_size_over(&self) -> bool {
        let upper = self.request.body().size_hint().upper().unwrap_or(u64::MAX);
        if upper > 1024 * 64 {
            return true;
        }
        false
    }

    // In case jason data, request_body would be like;
    // "{\"data\":\"<span class=\\\"classNameA\\\">Q&A</span>\"}"
    // whole data is closed by "".
    // " is escaped to \" because whole data is closed by "".
    // Because json data is closed by "", value inside "" is also escaped as \\\"
    async fn request_body(&mut self) -> Result<String, hyper::Error> {
        let request_body = hyper::body::to_bytes(self.request.body_mut()).await?;
        let request_body = String::from_utf8_lossy(&request_body).into_owned();
        Ok(request_body)
    }

    fn page(&mut self) -> Result<&mut page::Page, std::io::Error> {
        if self.page.is_none() {
            let page_path = self.source_path.to_string() + self.request.uri().path();

            match page::Page::from_path(&page_path) {
                Ok(page) => self.page.replace(page),
                Err(e) => {
                    eprintln!("not found: {}", page_path);
                    return Err(e);
                }
            };
        }
        Ok(self.page.as_mut().unwrap())
    }

    async fn handle(&mut self) -> Result<Response<Body>, hyper::Error> {
        if let Err(_) = self.page() {
            return Ok(res_404());
        }

        println!("{:?} {}", self.request.method(), self.request.uri().path());

        // handle method cases
        if self.request.method() == Method::POST {
            return self.handle_post().await;
        }

        if self.request.method() == Method::GET {
            return self.handle_get();
        }
        // temp
        Ok(Response::new("Hello, world".into()))
    }

    fn handle_get(&mut self) -> Result<Response<Body>, hyper::Error> {
        // println!("handle_get");

        match self.page().unwrap().source() {
            Ok(v) => return Ok(Response::new(v.to_vec().into())),
            Err(_) => return Ok(res_404()),
        }
    }

    async fn handle_post(&mut self) -> Result<Response<Body>, hyper::Error> {
        let wc_request = match self.wc_request() {
            Some(r) => r,
            None => return Ok(Response::new("No wc_request found".into())),
        };
        println!("wc_request: {:?}", wc_request);

        if self.body_size_over() {
            return Ok(res_body_too_big());
        }

        if wc_request == "json_save" {
            return self.json_save().await;
        }

        // temp
        Ok(Response::new("Hello, world".into()))
    }

    async fn json_save(&mut self) -> Result<Response<Body>, hyper::Error> {
        let request_body = self.request_body().await?;

        // To use json value in HTML
        // "value \" " : escape " by \"
        // & => &amp;
        // < > => &lt; &gt

        let json_post = match json::parse(&request_body) {
            Ok(page_json_parse) => page_json_parse,
            Err(_) => {
                return Ok(res_json_parse_failed());
            }
        };

        match self.page().unwrap().json_post_save(json_post) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("err: {:?}", e);
                return Ok(res_json_save_failed());
            }
        }

        // new rev no must be send back
        // or do not confirm rev matching

        // ~/projects/wc/wc_html/src/wc_handler/mod.rs
        Ok(Response::new(
            r#"{"res":"post_handle page_json_save"}"#.into(),
        ))
    }
}

// ==============

fn res_404() -> Response<Body> {
    println!("res_404");

    let mut not_found = Response::default();
    *not_found.status_mut() = StatusCode::NOT_FOUND;
    not_found
}

fn res_body_too_big() -> Response<Body> {
    let mut resp = Response::new(Body::from("Body too big"));
    *resp.status_mut() = hyper::StatusCode::PAYLOAD_TOO_LARGE;
    resp
}

fn res_json_parse_failed() -> Response<Body> {
    let mut resp = Response::new(Body::from("Failed to parse to json"));
    *resp.status_mut() = hyper::StatusCode::PARTIAL_CONTENT;
    resp
}

fn res_json_save_failed() -> Response<Body> {
    let mut resp = Response::new(Body::from("Failed to save json posted"));
    *resp.status_mut() = hyper::StatusCode::NOT_MODIFIED;
    resp
}

fn _monitor_req_info(request: &Request<Body>) {
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

    println!("method: {:?}", request.method());
    if request.method() == Method::GET {
        // println!("method: GET");
    }
    if request.method() == Method::POST {
        // println!("method: POST");
    }
}

// page data update
// Web -- Server
// file data to client
// wasm make html form data

// struct test {
//     dom: RcDom,
// }
