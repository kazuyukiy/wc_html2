use std::io::Result;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
// use tracing::info; //  event, instrument, span, Level debug,

mod js_css;
mod thread_pool;
// mod wc_handler;
mod wc_handler2;

/// stor_root: root path for storeage of the pages
// stor_root
pub fn wc_note(addr: &str, stor_root: &str, capa: usize) -> Result<TcpListener> {
    // Copy wc.js, wc.css to ./page/
    // Do it only once when start main()
    // if you change wc.js or wc.css, you may restart main() or copy it manulally
    js_css::setup();

    // wc_handler::system_ini();

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

    let response = wc_handler2::response(&mut stream, &stor_root);

    stream.write(&response).unwrap();
    stream.flush().unwrap();
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
