use std::io::Result;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

mod thread_pool;
mod wc_handler;

// page_root
pub fn wc_note(soket_add: &str, page_root: &str, capa: usize) -> Result<TcpListener> {
    wc_handler::system_ini();

    let listener = match TcpListener::bind(soket_add) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let pool = thread_pool::ThreadPool::new(capa);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(v) => v,
            Err(_) => continue,
        };

        let page_root = String::from(page_root);
        pool.execute(|| {
            handle_connection(stream, page_root);
        });
    }

    Ok(listener)
}

fn handle_connection(mut stream: TcpStream, page_root: String) {
    // Consider to reject access from wher not local
    // println!("lib.rs fn handle_connection cp0");

    let response = wc_handler::response(&mut stream, &page_root);
    stream.write(&response).unwrap();
    stream.flush().unwrap();
}
