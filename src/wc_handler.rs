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

    // DBG
    // if http_request.path() == "/favicon.ico" {
    //     return http_404();
    // }
    // info!("DBG calling handle_post");
    // return handle_post(&http_request, stor_root).unwrap_or(http_404());

    // DBG
    // if http_request.path() == "/Computing/go_move/go_move.html" {
    //     info!("DBG calling handle_page_move");
    //     handle_page_move(&http_request, stor_root);
    // }

    // DBG
    // if http_request.path_contains_rev() {
    //     info!("contains rev: {}", http_request.path());
    // }

    if method == "GET" {
        return handle_get(&http_request, stor_root).unwrap_or(http_404());
    }

    // Case http_request.path() ends with rev: abc.html.003
    // Ignore POST from page with rev no, those are backup file.
    if http_request.path_ends_with_rev() {
        info!("contains rev: {}", http_request.path());
        return http_404();
    }

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

    if wc_request == "href" {
        return handle_href(http_request);
    }

    if wc_request == "page_move" {
        return handle_page_move(http_request, stor_root);
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

    // let json_post = http_request.body_string().ok_or(())?;

    // // &str to JsonValue
    // // let json_post = json::parse(&json_post).or_else(|_| {
    // //     eprintln!("Failed to parse to json");
    // //     Err(())
    // // })?;
    // let json_post = match json::parse(&json_post) {
    //     Ok(v) => v,
    //     Err(_) => {
    //         eprintln!("Failed to parse to json");
    //         return Err(());
    //     }
    // };

    let json_post = http_request.body_json().ok_or(())?;

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
    let mut parent_page = page::Page::new(stor_root, http_request.path());

    let url = http_request.url().ok_or(())?;

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

    // let mut child_page = page::page_utility::child_page_new(&mut parent_page, url, title, href)?;
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

// fn handle_page_move_dbg(
//     http_request: &http_request::HttpRequest,
//     stor_root: &str,
// ) -> Result<Vec<u8>, ()> {
//     // DBG
//     info!("DBG set parent_url, dest_url");
//     let (parent_url, dest_url) = (
//         "http://127.0.0.1:3000/Computing/computing_iroiro.html",
//         "http://127.0.0.1:3000/Computing/move_to/move_to.html",
//     );

//     // //
//     if dest_url.len() == 0 {
//         return Err(());
//     }

//     // let mut page = page::Page::new(&stor_root, http_request.path());
//     let mut page = page::Page::new(&stor_root, "/Computing/move_from/move_from.html");

//     // let page_url = http_request.url().ok_or(())?;
//     let page_url =
//         url::Url::parse("http://127.0.0.1:3000/Computing/move_from/move_from.html").unwrap();

//     let parent_url = if parent_url.len() == 0 {
//         None
//     } else {
//         Some(page_url.join(parent_url).or(Err(()))?)
//     };

//     let dest_url = page_url.join(dest_url).or(Err(()))?;

//     let res = match page.page_move(page_url, dest_url, parent_url) {
//         // temp
//         Ok(_) => format!(r#"{{"Ok":"ok"}}"#),
//         Err(e) => format!(r#"{{"Err":"{}"}}"#, &e),
//     };

//     // DBG
//     info!("page_move_dbg: {}", &res);

//     Ok(http_ok(&res.as_bytes().to_vec()))
// }

fn handle_page_move(
    http_request: &http_request::HttpRequest,
    stor_root: &str,
) -> Result<Vec<u8>, ()> {
    // let json_post = http_request.body_json().ok_or(())?;

    // DBG
    // return handle_page_move_dbg(http_request, stor_root);

    let json_post = http_request.body_json().ok_or(())?;
    let parent_url = json_post["parent_url"].as_str().ok_or(())?.trim();
    let dest_url = json_post["dest_url"].as_str().ok_or(())?.trim();

    if dest_url.len() == 0 {
        return Err(());
    }

    let mut page = page::Page::new(&stor_root, http_request.path());
    let page_url = http_request.url().ok_or(())?;

    let parent_url = if parent_url.len() == 0 {
        None
    } else {
        Some(page_url.join(parent_url).or(Err(()))?)
    };

    let dest_url = page_url.join(dest_url).or(Err(()))?;

    let res = match page.page_move(page_url, dest_url, parent_url) {
        // temp
        // Ok(_) => format!(r#"{{"Ok":"ok"}}"#),
        Ok(_) => {
            let res = format!(r#"{{"Ok":"ok"}}"#);
            info!("page.page_move res: {}", res);
            res
        }
        Err(e) => format!(r#"{{"Err":"{}"}}"#, &e),
    };
    Ok(http_ok(&res.as_bytes().to_vec()))
}
