use httparse;
use std::io::Read;
use std::net::TcpStream;

pub fn response(stream: &mut TcpStream) -> Result<String, ()> {
    let stream_buf = match stream_read(stream) {
        Ok(b) => b,
        Err(_) => return Err(()),
    };

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    let body_offset = match req.parse(&stream_buf) {
        Ok(status) => match status {
            httparse::Status::Complete(pos) => pos,
            httparse::Status::Partial => return Err(()),
            // httparse::Status::Partial => {}
        },
        Err(_) => return Err(()),
    };

    match req.method {
        Some(r) => println!("request method: {:?}", r),
        None => (),
    }

    Ok(String::from("Hello"))
}

fn stream_read(stream: &mut TcpStream) -> Result<Vec<u8>, ()> {
    const MESSAGE_SIZE: usize = 1024;
    let mut read_buffer = [0u8; MESSAGE_SIZE];
    let mut stream_buffer: Vec<u8> = vec![];

    loop {
        match stream.read(&mut read_buffer) {
            Ok(bytes_read) => {
                stream_buffer.extend_from_slice(&read_buffer[..bytes_read]);
                if bytes_read < MESSAGE_SIZE {
                    break;
                }
            }
            Err(_) => return Err(()),
        }
    }

    Ok(stream_buffer)
}
