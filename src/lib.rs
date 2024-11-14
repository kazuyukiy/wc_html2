use std::io::Result;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
// use tracing::{info, info_span}; //  event, instrument, span, Level debug,

mod js_css;
mod page_upgrade_handle;
mod thread_pool;
mod wc_handler;

// #[macro_use]
// extern crate markup5ever;

/// addr: host and port ex: "127.0.0.1:3000"
/// stor_root: root path for storeage of the pages
/// capa: number of thread_pool
pub fn wc_note(addr: &str, stor_root: &str, capa: usize) -> Result<TcpListener> {
    // Copy the latest wc.js, wc.css to ./page/.
    // It is done only once when wc_note() is called.
    // If you change contents of wc.js or wc.css, you may recall wc_note() to apply the changes.
    js_css::setup();

    // page type upgrade
    let stor_root2 = stor_root.to_string();
    let upgrade_handle = std::thread::spawn(|| {
        let stor_root2 = stor_root2;
        page_upgrade_handle::pages_upgrade_handle(&stor_root2);
    });

    let listener = match TcpListener::bind(addr) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to bind: {:?}", e);
            return Err(e);
        }
    };

    println!("bind {}", addr);

    let pool = thread_pool::ThreadPool::new(capa);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(v) => v,
            Err(_) => continue,
        };

        let stor_root = String::from(stor_root);
        pool.execute(|| {
            handle_connection(stream, stor_root);
        });
    }

    // Handling listener does not wait end of upgrade_handles
    upgrade_handle.join().unwrap();

    Ok(listener)
}

fn handle_connection(mut stream: TcpStream, stor_root: String) {
    // Consider to reject access from wher not local

    let response = wc_handler::response(&mut stream, &stor_root);
    stream.write(&response).unwrap();
    stream.flush().unwrap();
}
