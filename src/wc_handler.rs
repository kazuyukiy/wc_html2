// use std::io::Read;
use std::net::TcpStream;
mod http_request; // does not work
mod page;

// Initialization
// fn system_ini() is to be called at beginning of lib.rs wc_node()
// so it will be done once at the biginning
// not every Tcp connection
//
// Copy wc.js, wc.css to
pub fn system_ini() {

    // under construction
}

pub fn response(stream: &mut TcpStream, page_root: &str) -> Vec<u8> {
    // dbg
    // println!("wc_handler fn response cp");

    let http_request = http_request::HttpRequest::new(stream);

    let mut page = match page(page_root, &http_request) {
        Ok(v) => v,
        Err(e) => return e,
    };

    let method = match http_request.method.as_ref() {
        Some(v) => v,
        None => return http_404(),
    };

    if method == "GET" {
        return handle_get(&mut page);
    }

    if method == "POST" {
        return handle_post(&mut page, &http_request);
    }

    // temp
    http_hello()
}

fn page(page_root: &str, http_request: &http_request::HttpRequest) -> Result<page::Page, Vec<u8>> {
    let path = match http_request.path.as_ref() {
        Some(v) => v,
        None => return Err(http_404()),
    };

    let page_path = page_root.to_string() + &path;

    let page = match page::Page::from_path(&page_path) {
        Ok(v) => v,
        Err(_) => return Err(http_404()),
    };

    Ok(page)
}

fn handle_get(page: &mut page::Page) -> Vec<u8> {
    match page.source() {
        Some(v) => http_ok(v),
        None => http_404(),
    }
}

fn handle_post(page: &mut page::Page, http_request: &http_request::HttpRequest) -> Vec<u8> {
    // case backup file
    if page.name_end_num() {
        return http_400();
    }

    let wc_request = match http_request.wc_request.as_ref() {
        Some(wc_request) => wc_request,
        None => return http_400(),
    };

    page.json_set();
    // page.rev() exists that means the file contains json data properly
    // otherwise no further processes
    if page.rev().is_none() {
        return http_400();
    }

    if wc_request == b"json_save" {
        return handle_json_save(page, http_request);
    }

    // temp
    http_404()
}

fn handle_json_save(page: &mut page::Page, http_request: &http_request::HttpRequest) -> Vec<u8> {
    let body = match http_request.body.as_ref() {
        Some(v) => v,
        None => return http_400(),
    };

    let json_post = match json::parse(&body) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Failed to parse to json");
            return http_400();
        }
    };

    json_post_save(page, json_post)
}

fn json_post_save(page: &mut page::Page, json_post: json::JsonValue) -> Vec<u8> {
    // if page_post.page_save_rev() in fn json_post_save of page::Page was done
    // in previous json_post_save(), it should return Err
    let _ = page.page_save_rev();

    let res_json = match page.json_post_save(json_post) {
        Ok(_) => r#"{"res":"post_handle page_json_save"}"#,
        Err(_) => r#"{"res":"failed to save page_json"}"#,
    };

    http_ok(&res_json.as_bytes().to_vec())
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

fn http_ok(contents: &Vec<u8>) -> Vec<u8> {
    http_form("200 OK", contents)
}

fn http_err(status: &str) -> Vec<u8> {
    http_form(status, &status.as_bytes().to_vec())
}

fn http_400() -> Vec<u8> {
    http_err("400 Bad Request.")
}

fn http_404() -> Vec<u8> {
    http_err("404 Not Found.")
} // end of fn http_404
