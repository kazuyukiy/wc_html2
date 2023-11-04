use std::io::Read;
use std::net::TcpStream;
// mod http_request; // does not work
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

// pub fn response(stream: &mut TcpStream, page_root: String) -> Vec<u8> {
pub fn response(stream: &mut TcpStream, page_root: &str) -> Vec<u8> {
    // dbg
    // println!("wc_handler fn response cp");

    // let mut stream_data = stream_read_to_end(stream);
    let stream_data = stream_read(stream);

    let mut headers = [httparse::EMPTY_HEADER; 64];
    let mut request = httparse::Request::new(&mut headers);

    let body_offset = match request.parse(&stream_data) {
        Ok(s) => match s {
            httparse::Status::Complete(l) => l,
            httparse::Status::Partial => stream_data.len(),
        },
        Err(_) => stream_data.len(),
    };

    // let (method, _page_path, mut page) = match method_path_page(page_root, &request) {
    let (method, mut page) = match method_page(page_root, &request) {
        Ok(v) => v,
        Err(e) => return e,
    };

    if method == "GET" {
        return handle_get(&mut page);
    }

    if method == "POST" {
        // let body = stream_data[body_offset..stream_data.len()].to_vec();
        let body = stream_data[body_offset..].to_vec();
        return handle_post(&mut page, &request, body);
    }

    // let _body = stream_data[body_offset..].to_vec();

    // temp
    http_hello()
}

fn stream_read_to_end(stream: &mut TcpStream) -> Vec<u8> {
    let mut stream_data: Vec<u8> = Vec::new();
    // issue read_to_end() does not return result
    let res = stream.read_to_end(&mut stream_data);

    match res {
        Ok(_) => (),
        Err(e) => eprintln!("wc_handler fn stream_read_to_end Err: {:?}", e),
    }

    stream_data
}

fn stream_read(stream: &mut TcpStream) -> Vec<u8> {
    // const MESSAGE_SIZE: usize = 5;
    const MESSAGE_SIZE: usize = 1024;
    let mut rx_bytes = [0u8; MESSAGE_SIZE];
    let mut recieved: Vec<u8> = vec![];

    loop {
        match stream.read(&mut rx_bytes) {
            Ok(bytes_read) => {
                recieved.extend_from_slice(&rx_bytes[..bytes_read]);
                if bytes_read < MESSAGE_SIZE {
                    break;
                }
            }

            Err(_) => {
                break;
            }
        }
    }

    recieved
}

// fn method<'a>(request: &'a httparse::Request) -> Result<&'a str, Vec<u8>> {
//     match request.method {
//         Some(v) => Ok(v),
//         None => Err(http_400()),
//     }
// }

// fn path<'a>(request: &'a httparse::Request) -> Result<&'a str, Vec<u8>> {
//     match request.path {
//         Some(v) => Ok(v),
//         None => Err(http_400()),
//     }
// }

// fn page<'a>(page_root: &str, request: &'a httparse::Request) -> Result<page::Page, Vec<u8>> {
//     let path = path(request)?;
//     let method = method(request)?;

//     let page_path = page_root.to_string() + path;

//     // method, pass
//     println!("{} {}", method, page_path);

//     let page = match page::Page::from_path(&page_path) {
//         Ok(v) => v,
//         Err(_) => return Err(http_404()),
//     };

//     Ok(page)
// }

// fn method_path_page is a bit complicated, but code using this function becomes simpler
// fn method(), fn page() make is simpler, but code colling those becomes bigger.
// fn method_path_page<'a>(
//     // page_root: String,
//     page_root: &str,
//     request: &'a httparse::Request,
// ) -> Result<(&'a str, String, page::Page), Vec<u8>> {
//     // println!("wc_handler fn method_path_page");

//     // println!("wc_handler fn method_path_page {:?}", request.method);

//     let method = match request.method {
//         Some(v) => v,
//         None => return Err(http_400()),
//     };

//     // println!("wc_handler fn method_path_page {}", method);

//     // let page_path = match request.path {
//     let path = match request.path {
//         Some(v) => v,
//         None => return Err(http_400()),
//     };

//     // let page_path = page_root.to_string() + page_path;
//     let page_path = page_root.to_string() + path;

//     // method, pass
//     println!("{} {}", method, page_path);

//     let page = match page::Page::from_path(&page_path) {
//         Ok(v) => v,
//         Err(_) => return Err(http_404()),
//     };

//     Ok((method, page_path, page))
// }

// fn method_page is a bit complicated, but code using this function becomes simpler
// fn method(), fn page() make is simpler, but code colling those becomes bigger.
fn method_page<'a>(
    // page_root: String,
    page_root: &str,
    request: &'a httparse::Request,
) -> Result<(&'a str, page::Page), Vec<u8>> {
    // println!("wc_handler fn method_page");

    // println!("wc_handler fn method_page {:?}", request.method);

    let method = match request.method {
        Some(v) => v,
        None => return Err(http_400()),
    };

    // println!("wc_handler fn method_page {}", method);

    // let page_path = match request.path {
    let path = match request.path {
        Some(v) => v,
        None => return Err(http_400()),
    };

    // let page_path = page_root.to_string() + page_path;
    let page_path = page_root.to_string() + path;

    // method, pass
    println!("{} {}", method, page_path);

    let page = match page::Page::from_path(&page_path) {
        Ok(v) => v,
        Err(_) => return Err(http_404()),
    };

    Ok((method, page))
}

fn handle_get(page: &mut page::Page) -> Vec<u8> {
    match page.source() {
        Some(v) => {
            //
            http_ok(v)
        }
        None => {
            // dbg
            // eprintln!("wc_handler.rs fn hadle_get err: {:?}", e);
            // return http_404();
            http_404()
        }
    }
}

fn handle_post(page: &mut page::Page, request: &httparse::Request, body: Vec<u8>) -> Vec<u8> {
    // case backup file
    if page.name_end_num() {
        return http_400();
    }

    let wc_request = match request.headers.iter().find(|&&h| h.name == "wc-request") {
        Some(header) => header.value,
        None => return http_400(),
    };

    if wc_request == b"json_save" {
        return handle_json_save(request, page, body);
    }

    // temp
    http_404()
}

fn handle_json_save(_request: &httparse::Request, page: &mut page::Page, body: Vec<u8>) -> Vec<u8> {
    // println!("wc_handler fn handle_json_save body: {:?}", body);

    let body = match String::from_utf8(body) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Failed to convert body to String");
            return http_400();
        }
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
    // dbg
    // println!("wc_handler.rs fn json_post_save");

    page.dom_set();
    page.json_set();
    if let Err(_) = page.page_save_rev() {
        eprintln!("");
    }

    page.json_post_save(json_post);

    // dbg
    // println!("wc_handler.rs fn json_post_save done");

    // temp
    http_404()
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
