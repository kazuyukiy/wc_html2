use super::wc_handler;
use tracing::error; //  error, event, instrument, span, Level debug,, info, info_span
                    // use super::wc_handler::page;
use wc_handler::page;
// use wc_handler::page::page_utility;
// use wc_handler::page::Page;

// pub fn pages_upgrade(stor_root: &str, page_path: &str) {
pub fn pages_upgrade(stor_root: &str) {
    // let page_path = "wc_top.html";
    let page_path = "/Computing/Html/html_basic.html";
    page_upgrade(stor_root, page_path);
    // if let Err(e) = page_upgrade(stor_root, page_path) {
    //     error!("{}", &e);
    // }
}

pub fn page_upgrade(stor_root: &str, page_path: &str) {
    let mut page = page::Page::new(&stor_root, page_path);
    if let Err(e) = page.upgrade() {
        error!("{}", &e);
    }
}
// pub fn page_upgrade_(stor_root: &str, page_path: &str) {
//     let mut page = page::Page::new(&stor_root, page_path);
//     let page_dom = match page.dom() {
//         Some(v) => v,
//         None => return,
//     };

//     let page_node = &page_dom.document;

//     // check if page type is the latest or to be upgraded.
//     // page_utility::json_from_dom(&page_node)

//     // json_value found in the current page style, not for upgrade
//     if page_utility::json_from_dom_span(page_node).is_some() {
//         return;
//     }

//     // json_value found in the script element.
//     if let Some(json_value) = page_utility::json_from_dom_script(page_node) {
//         //

//         // page_utility::page_system_version_upgrade(self)
//     }

//     // page::json_value, page::json, page::json_parse,
//     // page_utility::json_from_dom(dom)
//     // at json_from_dom, it convert html to page_json,
//     // in old page type case as well.
//     if let Some(page_json) = page.json_value() {
//         // let vec = page::page_utility::source_from_json(&page_json);
//         // return http_ok(&vec);
//         // return;
//     }
// }
