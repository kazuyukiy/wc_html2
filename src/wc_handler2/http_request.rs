use std::io::Read;
use std::net::TcpStream;
// use tracing::info; //  event, instrument, span, Level

pub struct HttpRequest {
    // method: Option<String>,
    // path: Option<String>,
    method: String,
    path: String,
    wc_request: Option<String>,
    host: Option<String>,
    body: Option<Vec<u8>>,
}

impl HttpRequest {
    pub fn from(stream: &mut TcpStream) -> Result<HttpRequest, ()> {
        let stream_data = stream_read(stream);
        // info!("fn from");
        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut request = httparse::Request::new(&mut headers);

        let body_offset = match request.parse(&stream_data) {
            Ok(s) => match s {
                httparse::Status::Complete(l) => Some(l),
                httparse::Status::Partial => None,
            },
            Err(_) => return Err(()),
        };

        // info!("fn from: stream_data: {:?}", stream_data);
        // info!("fn from cp1 request.path: {:?}", request.path);
        // info!("fn from cp1 body_offset: {:?}", body_offset);

        // request.path
        let path = match request.path {
            Some(path) => path.to_string(),
            None => return Err(()),
        };
        // if let Some(path) = request.path {
        //     http_request.path.replace(path.to_string());
        // } else {
        //     return Err(());
        // }

        // info!("fn from cp2 method:{:?}", request.method);
        // info!("fn from cp2 path:{:?}", request.path);

        // request.method
        let method = match request.method {
            Some(method) => method.to_string(),
            None => return Err(()),
        };
        // if let Some(method) = request.method {
        //     http_request.method.replace(method.to_string());
        // } else {
        //     return Err(());
        // }

        let mut http_request = HttpRequest {
            method,
            path,
            wc_request: None,
            host: None,
            body: None,
        };

        // GET
        if http_request.method() == "GET" {
            return Ok(http_request);
        }

        // request.headers
        // wc-request
        if let Some(v) = head_value(&request, "wc-request") {
            let vu8 = v.to_vec();
            if let Ok(v) = String::from_utf8(vu8) {
                http_request.wc_request.replace(v);
            };
        }

        // Host // ex.: 127.0.0.1:3000
        if let Some(v) = head_value(&request, "Host") {
            // let vu8 = v.to_vec();
            let v = v.to_vec();
            // if let Ok(v) = String::from_utf8(vu8) {
            if let Ok(v) = String::from_utf8(v) {
                http_request.host.replace(v);
            }
        }

        // body
        if body_offset.is_some() {
            let body = stream_data[body_offset.unwrap()..].to_vec();
            http_request.body.replace(body);
        };

        Ok(http_request)
    }

    // pub fn method(&self) -> Option<&str> {
    pub fn method(&self) -> &str {
        // self.method.as_ref().map(|v| v.as_str())
        // &self.method.as_ref().unwrap()
        &self.method
    }

    // pub fn path(&self) -> Option<&str> {
    pub fn path(&self) -> &str {
        // self.path.as_ref().map(|v| v.as_str())
        // self.path.as_ref().unwrap()
        &self.path
    }

    pub fn wc_request(&self) -> Option<&str> {
        self.wc_request.as_ref().map(|v| v.as_str())
    }

    pub fn body(&self) -> Option<&Vec<u8>> {
        self.body.as_ref()
    }

    pub fn body_string(&self) -> Option<String> {
        self.body
            .as_ref()
            .and_then(|v| Some(v.to_vec()))
            .and_then(|v| String::from_utf8(v).ok())
    }

    pub fn body_json(&self) -> Option<json::JsonValue> {
        let json_post = self.body_string()?;
        json::parse(&json_post).ok()
    }

    pub fn url(&self) -> Option<url::Url> {
        let host = self.host.as_ref()?;
        let path = &self.path;

        let url = format!("https://{}{}", host, path);
        url::Url::parse(&url).ok()
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

fn head_value<'a>(request: &'a httparse::Request, name: &str) -> Option<&'a [u8]> {
    match request.headers.iter().find(|&&h| h.name == name) {
        Some(header) => Some(header.value),
        None => None,
    }
}
