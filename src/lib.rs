use std::io::Result;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
// use tracing::{info, info_span}; //  event, instrument, span, Level debug,

mod js_css;
// mod page_upgrade;
mod thread_pool;
// mod wc_handler;
mod wc_handler;

// #[macro_use]
// extern crate markup5ever;

/// addr: host and port ex: "127.0.0.1:3000"
/// stor_root: root path for storeage of the pages
/// capa: number of thread_pool
pub fn wc_note(addr: &str, stor_root: &str, capa: usize) -> Result<TcpListener> {
    // Copy wc.js, wc.css to ./page/
    // Do it only once when start main()
    // if you change wc.js or wc.css, you may restart main() or copy it manulally
    js_css::setup();

    // page type upgrade
    let stor_root2 = stor_root.to_string();
    std::thread::spawn(|| {
        let stor_root2 = stor_root2;
        // let page_path = "wc_top.html";
        let page_path = "/Computing/Html/html_basic.html";
        let mut page = wc_handler::page::Page::new(&stor_root2, page_path);
        page.upgrade(true);
        // if let Err(e) = page.upgrade() {

        // page_upgrade::pages_upgrade(&stor_root2, "wc_top.html");
        // page_upgrade::pages_upgrade(&stor_root2);
    })
    .join()
    .unwrap();

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

    Ok(listener)
}

fn handle_connection(mut stream: TcpStream, stor_root: String) {
    // fn handle_connection(mut stream: TcpStream, stor_root: &str) {
    // Consider to reject access from wher not local
    // println!("lib.rs fn handle_connection cp0");

    // let _span_get = info_span!("HC").entered();

    // info!("fn handle_connection start");

    // wc_handler::stream_handle(&mut stream, &stor_root);
    // stream.flush().unwrap();
    // return;

    let response = wc_handler::response(&mut stream, &stor_root);
    stream.write(&response).unwrap();
    stream.flush().unwrap();

    // info!("fn handle_connection end");
}

// #[cfg(test)]
// mod test {

//     // #[traced_test]
//     #[test]
//     fn test_a() {
//         // info!("fn test_a");
//         println!("fn test_a");
//     }
// }
