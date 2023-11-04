fn main() {
    let addr = "127.0.0.1:3000";
    // let addr = "127.0.0.1:8080";
    let file_path = "./pages";
    let capa = 4;
    wc_note::wc_note(addr, file_path, capa);
}
