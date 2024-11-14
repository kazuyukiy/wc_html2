use std::net::TcpStream;
mod http_request;
pub mod page;
use tracing::{error, info, info_span};
//  error, event, instrument, span, Level debug,

pub fn response(stream: &mut TcpStream, stor_root: &str) -> Vec<u8> {
    match handle_stream(stream, stor_root) {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            http_404()
        }
    }
}

pub fn handle_stream(stream: &mut TcpStream, stor_root: &str) -> Result<Vec<u8>, String> {
    let http_request = match http_request::HttpRequest::from(stream) {
        Ok(v) => v,
        _ => {
            return Err("Failed to get http_request".to_string());
        }
    };

    let method = http_request.method();

    if method == "GET" {
        let _span_get = info_span!("GET").entered();
        info!("{}", http_request.path());

        // "/wc_top.html"
        // "/wc.js", "/wc.css", "/favicon.ico"
        return handle_get(&http_request, stor_root).or(Err("".to_string()));
    }

    if method == "POST" {
        let _span_post = info_span!("POST").entered();
        info!("{}", http_request.path());
        return handle_post(&http_request, stor_root);
    }

    // temp
    Ok(http_hello())
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

fn handle_get(http_request: &http_request::HttpRequest, stor_root: &str) -> Result<Vec<u8>, ()> {
    let mut page = page::Page::new(&stor_root, http_request.path());
    page.read().map_or(Err(()), |v| Ok(http_ok(v)))
}

fn handle_post(
    http_request: &http_request::HttpRequest,
    stor_root: &str,
) -> Result<Vec<u8>, String> {
    let Some(wc_request) = http_request.wc_request() else {
        return Err(format!("Failed to get wc_request: {}", http_request.path()));
    };

    info!("wc_request: {}", wc_request);

    if wc_request == "json_save" {
        return json_save(http_request, stor_root);
    }

    if wc_request == "page_new" {
        return page_new(http_request, stor_root);
    }

    if wc_request == "href" {
        return handle_href(http_request);
    }

    if wc_request == "page_move" {
        return handle_page_move(http_request, stor_root);
    }

    // temp
    Ok(http_hello())
}

/// Return Page instance if the page is much for POST.
/// Otherwise return None,
/// Sufix is htm*
/// Sufix does not number; wc_top.html.02
fn page_post(
    http_request: &http_request::HttpRequest,
    stor_root: &str,
) -> Result<page::Page, String> {
    if !http_request.path().contains(".htm") {
        return Err(format!("Not html: {}", http_request.path()));
    }

    let page = page::Page::new(stor_root, http_request.path());

    if page.is_end_with_rev() {
        return Err(format!(
            "It is a backup file, not for POST request: {}",
            http_request.path()
        ));
    }
    Ok(page)
}

fn json_post(http_request: &http_request::HttpRequest) -> Result<json::JsonValue, String> {
    http_request.body_json().ok_or(format!(
        "Failed to get request body in json: {}",
        http_request.path()
    ))
}

fn json_save(http_request: &http_request::HttpRequest, stor_root: &str) -> Result<Vec<u8>, String> {
    let mut page = page_post(http_request, stor_root)?;

    // The file not exist.
    // if page.read().is_err() {
    if page.source().is_none() {
        // return Err(());
        return Err(format!("Failed to read file: {}", page.file_path()));
    }

    // let json_post = http_request.body_json().ok_or(())?;
    // let json_post = http_request.body_json().ok_or(format!(
    //     "Failed to get request body in json: {}",
    //     http_request.path()
    // ))?;
    let json_post = json_post(http_request)?;

    // // Create static html
    // if let Err(e) = page::page_utility::page_dom_from_json::page_dom_from_json(&json_post) {
    //     error!("{}", e);
    // };

    let res: Vec<u8> = match page.json_replace_save(json_post) {
        Ok(_) => r#"{"res":"post_handle page_json_save"}"#.into(),
        Err(e) => {
            // eprintln!("fn json_save: {}", e);
            error!("fn json_save: {}", e);
            format!("{{\"res\":\"{}\"}}", e).into()
        }
    };

    // if let Ok(v) = std::str::from_utf8(&res) {
    //     info!("json_save: res: {}", v)
    // } else {
    // }

    Ok(http_ok(&res))
}

// fn page_new(http_request: &http_request::HttpRequest, stor_root: &str) -> Result<Vec<u8>, ()> {
fn page_new(http_request: &http_request::HttpRequest, stor_root: &str) -> Result<Vec<u8>, String> {
    let mut parent_page = page_post(http_request, stor_root)?;

    // let url = http_request.url().ok_or(())?;

    // title: title for new page
    // href: the location of the new page viewing from the parent.
    // let json_post = http_request.body_json().ok_or(())?;
    let json_post = json_post(http_request)?;

    // title
    let title = match json_post["title"].as_str() {
        Some(s) => s,
        None => {
            // eprintln!("title not found");
            // error!("title not found");
            // return Err(());
            return Err(format!("title not found: {}", http_request.path()));
            // return Ok(r#"{"res":"title not found"}"#.into());
        }
    };

    // href
    let href = match json_post["href"].as_str() {
        Some(s) => s,
        None => {
            // eprintln!("href not found");
            // return Err(());
            return Err(format!("href not found: {}", http_request.path()));
        }
    };

    // let mut child_page = page::page_utility::page_child_new(&mut parent_page, url, title, href)?;
    // let mut child_page = page::page_utility::page_child_new(&mut parent_page, title, href)?;
    let Ok(mut child_page) = page::page_utility::page_child_new(&mut parent_page, title, href)
    else {
        return Err(format!(
            "Failed to create page_child of {} on href: {}",
            http_request.path(),
            href
        ));
    };

    // child_page.dir_build()?;
    if child_page.dir_build().is_err() {
        return Err(format!(
            "Failed to create dir for : {}",
            http_request.path()
        ));
    }

    let res: Vec<u8> = match child_page.file_save_and_rev() {
        Ok(_) => r#"{"res":"post_handle page_new"}"#.into(),
        Err(_) => r#"{"res":"post_handle page_new failed"}"#.into(),
    };

    Ok(http_ok(&res))
}

// fn handle_href(http_request: &http_request::HttpRequest) -> Result<Vec<u8>, ()> {

fn handle_href(http_request: &http_request::HttpRequest) -> Result<Vec<u8>, String> {
    handle_href_temp(http_request)
}

// fn handle_href_temp(http_request: &http_request::HttpRequest) -> Result<Vec<u8>, ()> {
fn handle_href_temp(http_request: &http_request::HttpRequest) -> Result<Vec<u8>, String> {
    // let json_post = http_request.body_json().ok_or(())?;
    let json_post = json_post(http_request)?;

    // href
    let href = match json_post["href"].as_str() {
        Some(s) => s,
        None => {
            // eprintln!("href not found");
            // return Err(());
            let res = format!(r#"{{"Err":"href not found"}}"#);
            return Ok(http_ok(&res.as_bytes().to_vec()));
        }
    };

    // DBG
    // info!("fn handle_href_temp href_posted: {}", href);

    // {"dest":"href"}
    let res = format!(r#"{{"dest":"{}"}}"#, href);
    Ok(http_ok(&res.as_bytes().to_vec()))
}

fn handle_page_move(
    http_request: &http_request::HttpRequest,
    stor_root: &str,
    // ) -> Result<Vec<u8>, ()> {
) -> Result<Vec<u8>, String> {
    // let json_post = http_request.body_json().ok_or(())?;
    let json_post = json_post(http_request)?;
    let parent_url = json_post["parent_url"]
        .as_str()
        .ok_or(format!("Faild to get parent_url: {}", http_request.path()))?
        .trim();
    let dest_url = json_post["dest_url"]
        .as_str()
        .ok_or(format!("Failed to get dest_url: {}", http_request.path()))?
        .trim();

    if dest_url.len() == 0 {
        // return Err(());
        // format!(r#"{{"Err":"dest_url is empty"}}"#)
        let msg = r#"{{"Err":"dest_url is empty"}}"#;
        return Ok(http_ok(&msg.as_bytes().to_vec()));
    }

    let mut page = page_post(http_request, stor_root)?;
    let page_url = http_request
        .url()
        .ok_or(format!("Failed to get url: {}", http_request.path()))?;

    // info!("page_url: {}", page_url);

    let parent_url = if parent_url.len() == 0 {
        None
    } else {
        Some(
            page_url
                .join(parent_url)
                .or(Err(format!("Failed to join parent_url: {}", parent_url)))?,
        )
    };

    let dest_url = page_url.join(dest_url).or(Err(format!(
        "Failed to join destPurl: {}",
        http_request.path()
    )))?;

    let res = match page.page_move(page_url, dest_url, parent_url) {
        // temp
        // Ok(_) => format!(r#"{{"Ok":"ok"}}"#),
        // Ok(_) => format!(r#"{{"Ok":"ok"}}"#),
        Ok(_) => format!(r#"{{"res":"moved"}}"#),
        // {
        //     let res = format!(r#"{{"Ok":"ok"}}"#);
        //     info!("page.page_move res: {}", res);
        //     res
        // }
        Err(e) => format!(r#"{{"Err":"{}"}}"#, &e),
    };

    info!("{}", res);

    Ok(http_ok(&res.as_bytes().to_vec()))
}
