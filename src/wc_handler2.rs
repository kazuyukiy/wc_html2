use std::net::TcpStream;
mod http_request;
mod page;
use tracing::info; //  event, instrument, span, Level

pub fn response(stream: &mut TcpStream, page_root_path: &str) -> Vec<u8> {
    let http_request = match http_request::HttpRequest::from(stream) {
        Ok(v) => v,
        _ => return http_404(),
    };

    let method = http_request.method();
    info!("{} {}", method, http_request.path());

    if method == "GET" {
        return match handle_get(&http_request, page_root_path) {
            Ok(v) => v,
            Err(_) => http_404(),
        };
    }

    // temp
    http_hello()
}

fn http_hello() -> Vec<u8> {
    let contents = b"Hello".to_vec();
    http_ok(&contents)
}

fn http_form(status: &str, contents: &Vec<u8>) -> Vec<u8> {
    let header = format!(
        // "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n",
        status,
        contents.len()
    );

    [header.into_bytes(), contents.clone()].concat()
}

fn handle_get(http_request: &http_request::HttpRequest, page_root: &str) -> Result<Vec<u8>, ()> {
    let path = path(page_root, http_request.path());
    let mut page = page::Page::new(&path);

    page.read().map_or(Err(()), |v| Ok(http_ok(v)))

    // Ok(http_hello())
}

fn path(page_root: &str, path: &str) -> String {
    page_root.to_string() + path
}

fn page(http_request: &http_request::HttpRequest, page_root_path: &str) -> Result<page::Page, ()> {
    let path = path(page_root_path, http_request.path());
    let page = page::Page::new(&path);
    Ok(page)
}

fn http_ok(contents: &Vec<u8>) -> Vec<u8> {
    http_form("200 OK", contents)
}

fn http_err(status: &str) -> Vec<u8> {
    http_form(status, &status.as_bytes().to_vec())
}

fn _http_400() -> Vec<u8> {
    http_err("400 Bad Request.")
}

fn http_404() -> Vec<u8> {
    http_err("404 Not Found.")
}
