// use tracing::{instrument, Level}; // event, info, , span
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt::init();

    // let span = span!(Level::TRACE, "my span");
    // let _enter = span.enter();

    // info!("hooll");

    let addr = "127.0.0.1:8080";
    // let addr = "127.0.0.1:3000";
    let page_root_path = "./pages";
    let capa = 4;
    let _ = wc_note::wc_note(addr, page_root_path, capa);
}
