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

        // request.path
        let path = match request.path {
            Some(path) => path.to_string(),
            None => return Err(()),
        };

        // request.method
        let method = match request.method {
            Some(method) => method.to_string(),
            None => return Err(()),
        };

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

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    // pub fn path_contains_rev(&self) -> bool {
    pub fn path_ends_with_rev(&self) -> bool {
        // abc.html.003
        let reg = regex::Regex::new(r#"html.[0-9]+$"#).unwrap();
        reg.is_match(self.path())
        // self.path();

        // temp
        // balse
    }

    pub fn wc_request(&self) -> Option<&str> {
        self.wc_request.as_ref().map(|v| v.as_str())
    }

    fn _body(&self) -> Option<&Vec<u8>> {
        self.body.as_ref()
    }

    fn body_string(&self) -> Option<String> {
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
