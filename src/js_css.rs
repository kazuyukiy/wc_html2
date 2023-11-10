mod css_source;
mod js_source;

// Setup javascript and css file
pub fn setup() {
    for dtype in ["js", "css"] {
        handle_dtype(dtype);
    }
}

fn handle_dtype(dtype: &str) {
    // page/wc.js, page/wc.css
    let filename = "pages/wc.".to_string() + dtype;

    let source = match dtype {
        "js" => js_source::contents(),
        "css" => css_source::contents(),
        _ => "",
    };

    let _ = std::fs::write(&filename, &source);
}
