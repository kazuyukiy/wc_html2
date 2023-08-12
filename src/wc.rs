use httparse;
use std::cell::RefCell;
use std::io::Read;
use std::net::TcpStream;

pub fn response(stream: &mut TcpStream) -> Result<String, ()> {
    let mut stream_buffer: Vec<u8> = vec![];
    match stream_read(stream, &mut stream_buffer) {
        Ok(_) => (),
        Err(_) => return Err(()),
    };

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    let _body_offset = match req.parse(&stream_buffer) {
        Ok(status) => match status {
            httparse::Status::Complete(pos) => pos,
            httparse::Status::Partial => return Err(()),
        },
        Err(_) => return Err(()),
    };

    match req.method {
        Some(r) => println!("request method: {:?}", r),
        None => (),
    }

    Ok(String::from("Hello"))
}

fn stream_read(stream: &mut TcpStream, stream_buffer: &mut Vec<u8>) -> Result<(), ()> {
    const MESSAGE_SIZE: usize = 1024;
    let mut read_buffer = [0u8; MESSAGE_SIZE];
    // let mut stream_buffer: Vec<u8> = vec![];

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

    Ok(())
}

// pub fn response2(stream: &mut TcpStream) -> httparse::Request {
//     let mut stream_buffer: Vec<u8> = vec![];
//     stream_read(stream, &mut stream_buffer);

//     let mut headers = [httparse::EMPTY_HEADER; 16];
//     let mut req = httparse::Request::new(&mut headers);
//     let _body_offset = match req.parse(&stream_buffer) {
//         Ok(status) => match status {
//             httparse::Status::Complete(pos) => pos,
//             httparse::Status::Partial => 0,
//         },
//         Err(_) => 0,
//     };

//     match req.method {
//         Some(r) => println!("request method: {:?}", r),
//         None => (),
//     }

//     req
//     // Ok(String::from("Hello"))
// }

struct RequestBox<'h, 'b> {
    request: Option<httparse::Request<'h, 'b>>,
    // headers: [httparse::Header<'b>; 16],
    headers: RefCell<[httparse::Header<'b>; 16]>,
}

impl<'h, 'b> RequestBox<'h, 'b> {
    fn from() -> RequestBox<'h, 'b> {
        let mut request_box = RequestBox {
            request: None,
            // headers: [httparse::EMPTY_HEADER; 16],
            headers: RefCell::new([httparse::EMPTY_HEADER; 16]),
        };

        // httparse::Request::new(&mut request.headers);

        // .replace(httparse::Request::new(&mut request_box.headers));
        request_box
            .request
            // .replace(httparse::Request::new(*request_box.headers.borrow_mut()));
            .replace(httparse::Request::new(request_box.headers.get_mut()));

        request_box
    }
}
