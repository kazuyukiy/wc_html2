use std::fs;
use std::io::prelude::*;
use std::io::Result;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = wc_note::thread_pool::ThreadPool::new(4);

    // for stream in listener.incoming().take(2) {
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let (status_line, contents) = match wc_note::wc::response(&mut stream) {
        Ok(contents) => ("HTTP/1.1 200 OK", contents),
        Err(_) => ("HTTP/1.1 404 NOT FOUND", contents_404().unwrap()),
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn contents_404() -> Result<String> {
    fs::read_to_string("404.html")
    // Ok(String::new())
}

// fn handle_connection_(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).unwrap();

//     let get = b"GET / HTTP/1.1\r\n";
//     let sleep = b"GET /sleep HTTP/1.1\r\n";

//     let (status_line, filename) = if buffer.starts_with(get) {
//         ("HTTP/1.1 200 OK", "hello.html")
//     } else if buffer.starts_with(sleep) {
//         thread::sleep(Duration::from_secs(5));
//         ("HTTP/1.1 200 OK", "hello.html")
//     } else {
//         ("HTTP/1.1 404 NOT FOUND", "404.html")
//     };

//     let contents = fs::read_to_string(filename).unwrap();
//     let response = format!(
//         "{}\r\nContent-Length: {}\r\n\r\n{}",
//         status_line,
//         contents.len(),
//         contents
//     );

//     stream.write_all(response.as_bytes()).unwrap();
//     stream.flush().unwrap();
// }
