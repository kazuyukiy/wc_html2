use std::net::TcpStream;
mod http_request;
mod page;
use tracing::info; //  event, instrument, span, Level debug,

pub fn response(stream: &mut TcpStream, stor_root: &str) -> Vec<u8> {
    let http_request = match http_request::HttpRequest::from(stream) {
        Ok(v) => v,
        _ => return http_404(),
    };

    // info!("hots: {:?}", http_request.host.as_ref());

    let method = http_request.method();
    info!("{} {}", method, http_request.path());

    if method == "GET" {
        return handle_get(&http_request, stor_root).unwrap_or(http_404());
    }

    // dbg
    // info!("url: {}", http_request.url().unwrap());

    if method == "POST" {
        return handle_post(&http_request, stor_root).unwrap_or(http_404());
    }

    // temp
    http_hello()
}

// fn path(stor_root: &str, path: &str) -> String {
//     stor_root.to_string() + path
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

fn handle_get(http_request: &http_request::HttpRequest, stor_root: &str) -> Result<Vec<u8>, ()> {
    let mut page = page::Page::new(&stor_root, http_request.path());
    page.read().map_or(Err(()), |v| Ok(http_ok(v)))
}

fn handle_post(http_request: &http_request::HttpRequest, stor_root: &str) -> Result<Vec<u8>, ()> {
    let wc_request = if let Some(v) = http_request.wc_request() {
        v
    } else {
        return Err(());
    };
    info!("wc_request: {}", wc_request);

    if wc_request == "json_save" {
        return json_save(http_request, stor_root);
    }

    if wc_request == "page_new" {
        return page_new(http_request, stor_root);
    }

    // temp
    Ok(http_hello())
}

fn json_save(http_request: &http_request::HttpRequest, stor_root: &str) -> Result<Vec<u8>, ()> {
    let mut page = page::Page::new(stor_root, http_request.path());

    // The file not exist.
    if page.read().is_err() {
        return Err(());
    }

    let json_post = http_request.body_string().ok_or(())?;

    // &str to JsonValue
    // let json_post = json::parse(&json_post).or_else(|_| {
    //     eprintln!("Failed to parse to json");
    //     Err(())
    // })?;
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

fn page_new(http_request: &http_request::HttpRequest, stor_root: &str) -> Result<Vec<u8>, ()> {
    let mut p_page = page::Page::new(stor_root, http_request.path());

    let url = http_request.url().ok_or(())?;
    // info!("hots: {:?}", http_request.host.as_ref());

    // title: title for new page
    // href: the location of the new page viewing from the parent.
    let json_post = http_request.body_json().ok_or(())?;

    // title
    // json_post["title"].as_str();
    let title = match json_post["title"].as_str() {
        Some(s) => s,
        None => {
            eprintln!("title not found");
            return Err(());
        }
    };

    // href
    let href = match json_post["href"].as_str() {
        Some(s) => s,
        None => {
            eprintln!("href not found");
            return Err(());
        }
    };

    // info!("href: {}", href);

    let mut page_child = page::page_utility::page_child_new(&mut p_page, url, title, href)?;

    // info!("Got page_child");

    let res: Vec<u8> = match page_child.file_save_and_rev() {
        Ok(_) => r#"{"res":"post_handle page_new"}"#.into(),
        Err(_) => r#"{"res":"post_handle page_new failed"}"#.into(),
    };

    Ok(http_ok(&res))

    // page_child.file_save_and_rev().or(Err(()))

    // temp
    // Err(())
}
