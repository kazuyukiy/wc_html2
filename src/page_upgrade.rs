use super::wc_handler;
// use super::wc_handler::page;
use wc_handler::page;
// use wc_handler::page::Page;

// pub fn page_upgrade(stor_root: String, page_path: &str) {
pub fn page_upgrade(stor_root: &str, page_path: &str) {
    let mut page = page::Page::new(&stor_root, page_path);

    // check if page type is the latest or to be upgraded.

    // page::json_value, page::json, page::json_parse,
    // page_utility::json_from_dom(dom)
    // at json_from_dom, it convert html to page_json,
    // in old page type case as well.
    if let Some(page_json) = page.json_value() {
        // let vec = page::page_utility::source_from_json(&page_json);
        // return http_ok(&vec);
        // return;
    }
}
