// use crate::dom_to_json;
// use crate::page5;
//use crate::page_dom;

// Introduce page_json
// Return false in aborted.
// Return true when page_json is updated.
// pub fn update(page: &mut page5::Page) -> bool {
pub fn update(page: &mut super::Page) -> bool {
    // Not utf8, not wc page.
    if let None = page.file().content_str() {
        return false;
    }

    let page_json = match page.page_json.as_ref() {
        Some(v) => v.value(),
        None => return false,
    };

    // page_json exists;
    // if page.page_json.value()["system"].is_null() == false { return false; }
    if page_json["system"].is_null() == false {
        return false;
    }
    // Since wrong spel "syttem" was use.
    // if page.page_json.value()["syttem"].is_null() == false { return false; }
    if page_json["syttem"].is_null() == false {
        return false;
    }

    // save the current page as xxx.00
    let filename = page.file().filename().to_string() + ".00";
    let _res = page.file().save_as(&filename);

    let content_str = page.file().content_str();
    let page_dom = super::page_dom::PageDom::new(content_str.as_ref().unwrap());

    let doc = page_dom.dom.document;
    let page_json_value = super::dom_to_json::dom_to_json(&doc);

    // page.page_json.value_replace(page_json_value);
    page.page_json_replace(page_json_value);

    println!("page_system_update3_old_to_0_0_3");
    true
} // end of fn update
