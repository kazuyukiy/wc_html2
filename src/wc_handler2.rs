use std::net::TcpStream;
mod http_request;
mod page;
use tracing::info; //  event, instrument, span, Level debug,

pub fn response(stream: &mut TcpStream, page_root_path: &str) -> Vec<u8> {
    let http_request = match http_request::HttpRequest::from(stream) {
        Ok(v) => v,
        _ => return http_404(),
    };

    let method = http_request.method();
    info!("{} {}", method, http_request.path());

    if method == "GET" {
        return handle_get(&http_request, page_root_path).unwrap_or(http_404());
    }

    if method == "POST" {
        return handle_post(&http_request, page_root_path).unwrap_or(http_404());
    }

    // temp
    http_hello()
}

fn path(page_root: &str, path: &str) -> String {
    page_root.to_string() + path
}

// fn _page(http_request: &http_request::HttpRequest, page_root_path: &str) -> Result<page::Page, ()> {
//     let path = path(page_root_path, http_request.path());
//     let page = page::Page::new(&path);
//     Ok(page)
// }

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
}

fn handle_post(http_request: &http_request::HttpRequest, page_root: &str) -> Result<Vec<u8>, ()> {
    let wc_request = if let Some(v) = http_request.wc_request() {
        v
    } else {
        return Err(());
    };
    info!("wc_request: {}", wc_request);

    if wc_request == "json_save" {
        // return json_save(http_request, page_root);
        // return match json_save(http_request, page_root) {
        match json_save(http_request, page_root) {
            Ok(v) => {
                return Ok(v);
            }
            Err(_) => {
                return Err(());
            }
        };
    }

    // temp
    Ok(http_hello())
}

fn json_save(http_request: &http_request::HttpRequest, page_root: &str) -> Result<Vec<u8>, ()> {
    let path = &path(page_root, http_request.path());
    let mut page = page::Page::new(path);

    // The file not exist.
    if page.read().is_err() {
        return Err(());
    }

    let json_post = match http_request.body() {
        Some(v) => v.to_vec(), // &Vec to Vec
        None => return Err(()),
    };

    // Vec<u8> to String
    let json_post = match String::from_utf8(json_post) {
        Ok(v) => v,
        Err(_) => return Err(()),
    };

    // &str to JsonValue
    let json_post = match json::parse(&json_post) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Failed to parse to json");
            return Err(());
        }
    };

    let res: Vec<u8> = match page.json_replace_save(json_post) {
        Ok(_) => r#"{"res":"post_handle page_json_save"}"#.into(),
        Err(e) => {
            eprintln!("fn json_save: {}", e);
            format!("{{\"res\":\"{}\"}}", e).into()
        }
    };
    Ok(http_ok(&res))
}
