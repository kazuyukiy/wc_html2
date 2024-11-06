// use std::io::Write;
use std::net::TcpStream;
// use std::thread;
// use std::time::Duration;
mod http_request;
pub mod page;
use tracing::{error, info, info_span}; //  error, event, instrument, span, Level debug,

// pub fn stream_handle(stream: &mut TcpStream, stor_root: &str) {
//     //

//     // stream_handle2(stream, stor_root);
//     // stream.flush().unwrap();
//     // return;

//     // info!("stream_handle");
//     let response = response(stream, stor_root);
//     stream.write(&response).unwrap();
//     stream.flush().unwrap();
//     // info!("flush unwrapped");
// }

// fn stream_handle2(stream: &mut TcpStream, stor_root: &str) {
//     //

//     let http_request = match http_request::HttpRequest::from(stream) {
//         Ok(v) => v,
//         _ => {
//             // DBG
//             error!("Failed to get http_request");
//             stream.write(&http_404()).unwrap();
//             return;
//             // return http_404();
//         }
//     };

//     let method = http_request.method();

//     if method == "GET" {
//         let _span_get = info_span!("GET").entered();
//         info!("{}", http_request.path());

//         // DBG
//         let mut debug_mode = false;
//         if http_request.path().contains(".htm") {
//             debug_mode = true;
//             info!("path contains htm");
//         } else {
//             info!("path not contains htm");
//         }
//         // "/wc.js", "/wc.css", "/favicon.ico"
//         if debug_mode {
//             info!("debug_mode: {}", debug_mode);
//             // Create the page with contents in html previously
//             // before drawn by javascript.
//             // in debug mode
//             let mut page = page::Page::new(&stor_root, http_request.path());
//             if let Some(page_json) = page.json_value() {
//                 let vec = page::page_utility::source_from_json(&page_json);

//                 // return http_ok(&vec);
//                 stream.write(&http_ok(&vec)).unwrap();
//                 stream.flush().unwrap();

//                 // DBG
//                 info!("wait for 10sec.");

//                 // DBG
//                 thread::sleep(Duration::from_millis(500));

//                 // DBG
//                 info!("after write and 10sec.");

//                 return;
//             }
//         }

//         // return handle_get(&http_request, stor_root).unwrap_or(http_404());
//         let res = handle_get(&http_request, stor_root).unwrap_or(http_404());
//         stream.write(&res).unwrap();
//         return;
//     }

//     // Case http_request.path() ends with rev eg; abc.html.003
//     // Ignore POST from page with rev no, those are backup file.
//     //

//     // Case http_request.path() ends with rev eg; abc.html.003
//     // that is a backup file,
//     // inore POST request from the page,
//     //
//     // (old) http_request.path_ends_with_rev()
//     // let reg = regex::Regex::new(r#"html.[0-9]+$"#).unwrap();
//     // if reg.is_match(http_request.path()) {
//     //     info!("contains rev: {}", http_request.path());
//     //     return http_404();
//     // }
//     let page = page::Page::new(stor_root, http_request.path());
//     if page.is_end_with_rev() {
//         // return http_404();
//         stream.write(&http_404()).unwrap();
//         return;
//     }

//     if method == "POST" {
//         // return handle_post(&http_request, stor_root).unwrap_or(http_404());
//         let res = handle_post(&http_request, stor_root).unwrap_or(http_404());
//         stream.write(&res).unwrap();
//         return;
//     }

//     // temp
//     // http_hello()
//     stream.write(&http_hello()).unwrap();
// }

pub fn response(stream: &mut TcpStream, stor_root: &str) -> Vec<u8> {
    // info!("response");

    let http_request = match http_request::HttpRequest::from(stream) {
        Ok(v) => v,
        _ => {
            // DBG
            info!("Failed to get http_request");
            return http_404();
        }
    };

    let method = http_request.method();

    if method == "GET" {
        let _span_get = info_span!("GET").entered();
        info!("{}", http_request.path());

        // DBG
        let mut debug_mode = false;
        if http_request.path().contains(".htm") {
            debug_mode = true;
            // info!("path contains htm");
        } else {
            info!("DBG path not contains htm");
        }

        // "/wc.js", "/wc.css", "/favicon.ico"
        if debug_mode {
            info!("debug_mode: {}", debug_mode);
            // Create the page with contents in html previously
            // before drawn by javascript.
            // in debug mode
            let mut page = page::Page::new(&stor_root, http_request.path());
            if let Some(page_json) = page.json_value() {
                let vec = page::page_utility::source_from_json(&page_json);
                return http_ok(&vec);
            }
        }

        return handle_get(&http_request, stor_root).unwrap_or(http_404());
    }

    // Case http_request.path() ends with rev eg; abc.html.003 (backup file)
    // Ignore POST from those pages.
    let page = page::Page::new(stor_root, http_request.path());
    if page.is_end_with_rev() {
        return http_404();
    }

    if method == "POST" {
        return handle_post(&http_request, stor_root).unwrap_or(http_404());
    }

    // temp
    http_hello()
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

fn handle_post(http_request: &http_request::HttpRequest, stor_root: &str) -> Result<Vec<u8>, ()> {
    let _span_get = info_span!("POST").entered();
    info!("{}", http_request.path());

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

    if wc_request == "href" {
        return handle_href(http_request);
    }

    if wc_request == "page_move" {
        return handle_page_move(http_request, stor_root);
    }

    // if wc_request == "page_upgrade" {
    //     return handle_page_upgrade(http_request, stor_root);
    // }

    // temp
    Ok(http_hello())
}

fn json_save(http_request: &http_request::HttpRequest, stor_root: &str) -> Result<Vec<u8>, ()> {
    let mut page = page::Page::new(stor_root, http_request.path());

    // The file not exist.
    if page.read().is_err() {
        return Err(());
    }

    let json_post = http_request.body_json().ok_or(())?;

    // // Create static html
    // if let Err(e) = page::page_utility::page_dom_from_json::page_dom_from_json(&json_post) {
    //     error!("{}", e);
    // };

    let res: Vec<u8> = match page.json_replace_save(json_post) {
        Ok(_) => r#"{"res":"post_handle page_json_save"}"#.into(),
        Err(e) => {
            eprintln!("fn json_save: {}", e);
            format!("{{\"res\":\"{}\"}}", e).into()
        }
    };

    // if let Ok(v) = std::str::from_utf8(&res) {
    //     info!("json_save: res: {}", v)
    // } else {
    // }

    Ok(http_ok(&res))
}

fn page_new(http_request: &http_request::HttpRequest, stor_root: &str) -> Result<Vec<u8>, ()> {
    let mut parent_page = page::Page::new(stor_root, http_request.path());

    let url = http_request.url().ok_or(())?;

    // title: title for new page
    // href: the location of the new page viewing from the parent.
    let json_post = http_request.body_json().ok_or(())?;

    // title
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

    let mut child_page = page::page_utility::page_child_new(&mut parent_page, url, title, href)?;

    child_page.dir_build()?;

    let res: Vec<u8> = match child_page.file_save_and_rev() {
        Ok(_) => r#"{"res":"post_handle page_new"}"#.into(),
        Err(_) => r#"{"res":"post_handle page_new failed"}"#.into(),
    };

    Ok(http_ok(&res))
}

fn handle_href(http_request: &http_request::HttpRequest) -> Result<Vec<u8>, ()> {
    handle_href_temp(http_request)
}

fn handle_href_temp(http_request: &http_request::HttpRequest) -> Result<Vec<u8>, ()> {
    let json_post = http_request.body_json().ok_or(())?;

    // href
    let href = match json_post["href"].as_str() {
        Some(s) => s,
        None => {
            eprintln!("href not found");
            return Err(());
        }
    };

    // DBG
    info!("fn handle_href_temp href_posted: {}", href);

    // {"dest":"href"}
    let res = format!(r#"{{"dest":"{}"}}"#, href);
    Ok(http_ok(&res.as_bytes().to_vec()))
}

fn handle_page_move(
    http_request: &http_request::HttpRequest,
    stor_root: &str,
) -> Result<Vec<u8>, ()> {
    let json_post = http_request.body_json().ok_or(())?;
    let parent_url = json_post["parent_url"].as_str().ok_or(())?.trim();
    let dest_url = json_post["dest_url"].as_str().ok_or(())?.trim();

    if dest_url.len() == 0 {
        return Err(());
    }

    let mut page = page::Page::new(&stor_root, http_request.path());
    let page_url = http_request.url().ok_or(())?;

    // info!("page_url: {}", page_url);

    let parent_url = if parent_url.len() == 0 {
        None
    } else {
        Some(page_url.join(parent_url).or(Err(()))?)
    };

    let dest_url = page_url.join(dest_url).or(Err(()))?;

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

fn handle_page_upgrade(
    http_request: &http_request::HttpRequest,
    stor_root: &str,
) -> Result<Vec<u8>, ()> {
    let json_post = http_request.body_json().ok_or(())?;

    let url_str = json_post["upgrade_url"].as_str().ok_or(())?.trim();
    // DBG
    info!("url_str: {}", url_str);
    let url = match url::Url::parse(&url_str) {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to parse url: {:?}", e);

            let res = format!(r#"{{"res":"{:?}"}}"#, e);
            return Ok(http_ok(&res.as_bytes().to_vec()));
            // return Ok(http_ok(&e.to_string().as_bytes().to_vec()));
        }
    };

    // info!("upgrade url.path: {}", url.path());
    let mut page = page::Page::new(stor_root, url.path());
    let res = match page.upgrade(false) {
        Ok(_) => "upgraded".to_string(),
        // Err(e) => e.to_string(),
        Err(e) => e,
    };

    //    fn page_system_version_upgrade(&mut self, url: url::Url) -> Result<(), String> {

    // let res = format!(r#"{{"res":"upgraded"}}"#);
    let res = format!(r#"{{"res":"{}"}}"#, res);

    Ok(http_ok(&res.as_bytes().to_vec()))
}
