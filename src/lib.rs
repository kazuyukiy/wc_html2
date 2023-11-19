use std::io::Result;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

mod js_css;
mod thread_pool;
mod wc_handler;

// page_root_path
pub fn wc_note(soket_add: &str, page_root_path: &str, capa: usize) -> Result<TcpListener> {
    // Copy wc.js, wc.css to ./page/
    // Do it only once when start main()
    // if you change wc.js or wc.css, you may restart main() or copy it manulally
    js_css::setup();

    wc_handler::system_ini();

    let listener = match TcpListener::bind(soket_add) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to bind: {:?}", e);
            return Err(e);
        }
    };

    println!("bind {}", soket_add);

    let pool = thread_pool::ThreadPool::new(capa);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(v) => v,
            Err(_) => continue,
        };

        let page_root_path = String::from(page_root_path);
        pool.execute(|| {
            handle_connection(stream, page_root_path);
        });
    }

    Ok(listener)
}

fn handle_connection(mut stream: TcpStream, page_root_path: String) {
    // Consider to reject access from wher not local
    // println!("lib.rs fn handle_connection cp0");

    let response = wc_handler::response(&mut stream, &page_root_path);
    stream.write(&response).unwrap();
    stream.flush().unwrap();
}
