fn main() {
    let addr = "127.0.0.1:3000";
    // let addr = "127.0.0.1:8080";
    let page_root_path = "./pages";
    let capa = 4;
    let _ = wc_note::wc_note(addr, page_root_path, capa);
}
