use std::io::Read;
use std::net::TcpStream;

pub struct HttpRequest {
    // pub method: Option<String>,
    method: Option<String>,
    // pub path: Option<String>,
    path: Option<String>,
    // pub body: Option<String>,
    body: Option<String>,
    // pub host: Option<String>,
    host: Option<String>,
    // pub wc_request: Option<Vec<u8>>,
    // wc_request: Option<Vec<u8>>,
    wc_request: Option<String>,
}

// impl<'h, 'b> HttpRequest<'h, 'b> {
impl<'h, 'b> HttpRequest {
    pub fn new(stream: &mut TcpStream) -> HttpRequest {
        let stream_data = stream_read(stream);

        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut request = httparse::Request::new(&mut headers);
        let body_offset = match request.parse(&stream_data) {
            Ok(s) => match s {
                httparse::Status::Complete(l) => Some(l),
                httparse::Status::Partial => None,
            },
            Err(_) => None,
        };

        let method = match request.method {
            Some(v) => Some(v.to_string()),
            None => None,
        };

        let path = match request.path {
            Some(v) => Some(v.to_string()),
            None => None,
        };

        let mut http_request = HttpRequest {
            method,
            path,
            body: None,
            host: None,
            wc_request: None,
        };

        if http_request.method.is_some() && body_offset.is_some() {
            let method = http_request.method.as_ref().unwrap();
            // on case method is POST, set body and wc_request
            if method == "POST" {
                let body_offset = body_offset.unwrap();
                // http_request.body have a value when method is "POST"
                body_set(&mut http_request, body_offset, &stream_data);

                // http_request.host have a value when method is "POST"
                host_set(&mut http_request, &request);
                // http_request.wc_request have a value when method is "POST"
                wc_request_set(&mut http_request, &request);
            }
        }

        print_method_path(&http_request);

        http_request
    }

    pub fn method(&self) -> Option<&str> {
        match self.method.as_ref() {
            Some(r) => Some(&r),
            None => None,
        }
    }

    pub fn path(&self) -> Option<&str> {
        match self.path.as_ref() {
            Some(r) => Some(&r),
            None => None,
        }
    }

    // call fn body_set() before use body data
    fn _body(&self) -> Option<&str> {
        match self.body.as_ref() {
            Some(v) => Some(&v),
            None => None,
        }
    }

    pub fn body_json(&self) -> Option<json::JsonValue> {
        let body = match self.body.as_ref() {
            Some(v) => v,
            None => return None,
        };

        match json::parse(&body) {
            Ok(v) => Some(v),
            Err(_) => {
                eprintln!("Failed to parse to json");
                None
            }
        }
    }

    pub fn _host(&self) -> Option<&str> {
        match self.host.as_ref() {
            Some(r) => Some(&r),
            None => None,
        }
    }

    pub fn wc_request(&self) -> Option<&str> {
        match self.wc_request.as_ref() {
            Some(r) => {
                //
                Some(&r)
            }
            None => None,
        }
    }

    pub fn url(&self) -> Option<url::Url> {
        let host = self.host.as_ref()?;
        let path = self.path.as_ref()?;

        let url = format!("https://{}{}", host, path);
        match url::Url::parse(&url) {
            Ok(u) => Some(u),
            Err(_) => None,
        }
    }
}

fn stream_read(stream: &mut TcpStream) -> Vec<u8> {
    // const MESSAGE_SIZE: usize = 5;
    const MESSAGE_SIZE: usize = 1024;
    let mut rx_bytes = [0u8; MESSAGE_SIZE];
    let mut stream_data: Vec<u8> = vec![];

    loop {
        match stream.read(&mut rx_bytes) {
            Ok(bytes_read) => {
                stream_data.extend_from_slice(&rx_bytes[..bytes_read]);
                if bytes_read < MESSAGE_SIZE {
                    break;
                }
            }

            Err(_) => {
                break;
            }
        }
    }

    stream_data
}

fn _stream_read_to_end(stream: &mut TcpStream) -> Vec<u8> {
    let mut stream_data: Vec<u8> = Vec::new();
    // issue read_to_end() does not return result
    let res = stream.read_to_end(&mut stream_data);

    match res {
        Ok(_) => (),
        Err(e) => eprintln!("wc_handler fn stream_read_to_end Err: {:?}", e),
    }

    stream_data
}

fn body_set(http_request: &mut HttpRequest, body_offset: usize, stream_data: &Vec<u8>) {
    let body = stream_data[body_offset..].to_vec();

    let body = match String::from_utf8(body) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Failed to convert body to String");
            return;
        }
    };

    http_request.body.replace(body);
}

fn head_value<'a>(request: &'a httparse::Request, name: &str) -> Option<&'a [u8]> {
    match request.headers.iter().find(|&&h| h.name == name) {
        Some(header) => Some(header.value),
        None => None,
    }
}

fn wc_request_set(http_request: &mut HttpRequest, request: &httparse::Request) {
    match head_value(request, "wc-request") {
        Some(v) => {
            // http_request.wc_request.replace(v.to_vec());

            let vu8 = v.to_vec();
            match String::from_utf8(vu8) {
                Ok(v) => {
                    http_request.wc_request.replace(v);
                }
                Err(_) => (),
            }
        }
        None => (),
    }

    // match request.headers.iter().find(|&&h| h.name == "wc-request") {
    //     Some(header) => {
    //         http_request.wc_request.replace(header.value.to_vec());
    //     }
    //     None => (),
    // }
}

fn host_set(http_request: &mut HttpRequest, request: &httparse::Request) {
    match head_value(request, "Host") {
        Some(v) => {
            let host = match String::from_utf8(v.to_vec()) {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("Failed to convert host to String");
                    return;
                }
            };
            http_request.host.replace(host);
        }
        None => (),
    }
    // match request.headers.iter().find(|&&h| h.name == "Host") {
    //     Some(v) => {
    //         let host = v.value.to_vec();
    //         let host = match String::from_utf8(host) {
    //             Ok(v) => v,
    //             Err(_) => {
    //                 eprintln!("Failed to convert host to String");
    //                 return;
    //             }
    //         };
    //         http_request.host.replace(host);
    //     }
    //     None => (),
    // }
}

//

fn print_method_path(http_request: &HttpRequest) {
    let method = match http_request.method.as_ref() {
        Some(v) => v,
        None => return,
    };

    let path = match http_request.path.as_ref() {
        Some(v) => v,
        None => return,
    };
    // if http_request.method.is_none() {
    //     return;
    // }

    println!("{} {}", method, path);
}
