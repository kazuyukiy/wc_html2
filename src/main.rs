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

    // DBG
    // let page_path = "/Computing/Html/html_basic.html";
    // let page_path = "/Computing/computing_iroiro.html";
    // let page_path = "/Computing/computing_index.html";
    // let page_path = "/Computing/windows/windows10/windows10openssh.html";
    // let page_path = "/Computing/Linux/Package/Yum/linux_yum_index.html";
    // let page_path = "/Computing/Language/computer_language_index.html";
    // let page_path = "/pages/Computing/Windows/windows10/windows10openssh.html";

    let page_top_path = "/wc_top.html";

    let capa = 4;
    let _ = wc_note::wc_note(addr, page_root_path, page_top_path, capa);
}
