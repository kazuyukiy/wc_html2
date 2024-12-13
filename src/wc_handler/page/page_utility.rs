use super::super::super::page_upgrade_handle::Upres;
use super::Page;
use html5ever::serialize::SerializeOpts;
use markup5ever_rcdom::{Handle, Node, NodeData, RcDom, SerializableHandle};
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;
use tracing::{error, info}; // {event, info, instrument, span, Level, Node}
mod dom_utility;
mod json_from_dom_html;
pub mod page_dom_from_json;
// mod page_form_update;
mod page_move;
mod page_upgrade;
pub use super::page_json;
pub use page_move::page_move;
pub mod page_backup_delete;
pub mod page_mainte;

pub fn file_path(stor_root: &str, page_path: &str) -> String {
    // String + "." + &str
    stor_root.to_string() + page_path
}

pub fn page_url(page: &mut super::Page) -> Result<url::Url, String> {
    let local_host = "127.0.0.1";
    let page_path = page.page_path();
    let url = format!("http://{}{}", local_host, page_path);
    url::Url::parse(&url).or_else(|e| Err(format!("{}", e)))
}

pub fn fs_write(file_path: &str, contents: &Vec<u8>) -> Result<String, String> {
    std::fs::write(&file_path, contents)
        .and(Ok(file_path.to_string()))
        .or_else(|e| Err(e.to_string()))
}

/// Create dir of the path
// pub fn dir_build(path: &std::path::Path, recursive: bool) -> Result<(), ()> {
pub fn dir_build(path: &std::path::Path, recursive: bool) -> Result<String, String> {
    // path  : abc/def/ghi.html (Contains a file name.)
    // parent: abc/def (remain only directory path.)
    let parent = path
        .parent()
        .ok_or(format!("Failed to get parent: {:?}", path))?;

    // This count() counts depth of directory.
    // Consider how avoid too match deep directorys making.

    // Already exists.
    if let Ok(true) = parent.try_exists() {
        return Ok(format!("Already exists: {:?}", path.parent()));
    }

    // let parent_path = parent.to_str().ok_or(())?;
    let parent_path = parent
        .to_str()
        .ok_or(format!("Failed to get parent in str: {:?}", parent))?;
    match std::fs::DirBuilder::new()
        .recursive(recursive)
        .create(parent_path)
    {
        Ok(_) => {
            info!("dir created: {}", parent_path);
            Ok(format!("dir created: {}", parent_path))
        }
        Err(_) => {
            error!("Failed to create dir: {}", parent_path);
            Err(format!("Failed to create dir: {}", parent_path))
        }
    }
}

pub fn to_dom(source: &str) -> RcDom {
    dom_utility::to_dom(source)
}

pub fn to_dom_parts(source: &str) -> Vec<Rc<Node>> {
    dom_utility::to_dom_parts(source)
}

fn json_parse(str: &str) -> Option<json::JsonValue> {
    match json::parse(str) {
        Ok(v) => Some(v),
        Err(e) => {
            error!("Failed to parse json: {}", e);
            None
        }
    }
}

/// Get json text data from span element and return it as JsonValue
/// If span with json data is not found, get data from script element (old style)
/// or parse html data.
pub fn json_from_dom(page_node: &Handle) -> Option<json::JsonValue> {
    // current page stype, json value is in span element.
    // <span id="page_json_str" style="display: none"></span>
    let mut json_value = json_from_dom_span(page_node);

    // The script in below may not need since page_upgrade is done in lib.rs.
    // All pages upgraded should have json_value in span element.

    // old stype, json value is in scritp element.
    // <script type="text/javascript" class="page_json">let page_json = {}</script>
    if json_value.is_none() {
        json_value = json_from_dom_script(page_node);
    }

    // old page stype, no json value in the page.
    // Create json value parsing html data.
    // But the page structure of html is different from the current html page.
    if json_value.is_none() {
        json_value = json_from_dom_html(page_node);
    }

    json_value
}

pub fn json_from_dom_span(page_node: &Handle) -> Option<json::JsonValue> {
    // json in span
    let json_str = span_json_str(page_node)?;
    json_parse(&json_str)
}

/// Get page_json in string from span element
fn span_json_str(page_node: &Rc<Node>) -> Option<String> {
    let span = dom_utility::get_span_json(page_node)?;

    let children = span.children.borrow();
    if children.len() == 0 {
        eprintln!("Failed, json contents not found in the span element");
        return None;
    }

    let json_str = match &children[0].data {
        NodeData::Text { contents } => contents,
        _ => {
            error!("Failed to get json data in the span element");
            return None;
        }
    };
    let str = json_str.borrow().to_string();
    Some(str)
}

/// <script type="text/javascript" class="page_json">let page_json = {}</script>
pub fn json_from_dom_script(page_node: &Handle) -> Option<json::JsonValue> {
    // json in script
    let json_str = script_json_str(page_node)?;
    json_parse(&json_str)
}

/// <script type="text/javascript" class="page_json">let page_json = {}</script>
fn script_json_str(page_dom: &Rc<Node>) -> Option<String> {
    let script_node = dom_utility::get_script_json(&page_dom)?;
    for child in script_node.children.borrow().iter() {
        let content = match &child.data {
            NodeData::Text { contents } => contents.borrow(),
            _ => {
                continue;
            }
        };
        // (?s) includes new lines.
        let reg = regex::Regex::new(r#"(?s)\s*page_json\s*=\s*(\{.+\})\s*$"#).unwrap();
        let Some(caps) = reg.captures(&content) else {
            continue;
        };
        return Some(caps[1].to_string());
    }
    None
}

/// Convert page_dom that represent page contents, not page data in json as text,
/// to page_json data
fn json_from_dom_html(page_node: &Handle) -> Option<json::JsonValue> {
    json_from_dom_html::json_from_dom_html(page_node)
}

// Contains body onload="bodyOnload()"
fn page_html_plain() -> &'static str {
    // On static page, wc.js file may not be imported. In that case, internal script function bodyOnload avoid no function error.
    // the internal function bodyOnload is written before script tag with src for import so the function will be overwritten as its import.
    // But it is not sure this script tag order work well for every browser type.
    // To be secure, better to make a script that confirm existsnce of the function and handle it.
    r#"<!DOCTYPE html><html><head><title></title><meta charset="UTF-8"></meta><script>function bodyOnload () {}</script><script src="/wc.js"></script>
    <link rel="stylesheet" href="/wc.css"></link>
    <style type="text/css"></style>
</head><body onload="bodyOnload()"><span id="page_json_str" style="display: none"></span></body></html>
"#
}

/// Create a page source from json value.
pub fn source_from_json(page_path: &str, page_json: &json::JsonValue) -> Vec<u8> {
    let page_dom = page_dom_from_json::page_dom_from_json(page_path, page_json);
    if page_dom.is_err() {
        return vec![];
    }

    // dom.document
    match dom_serialize(page_dom.unwrap().document) {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to get source from json: {}", e);
            vec![]
        }
    }
}

fn dom_serialize(node: Rc<Node>) -> std::result::Result<Vec<u8>, std::io::Error> {
    // let sh = SerializableHandle::from(dom.document);
    let sh = SerializableHandle::from(node);
    let mut page_bytes = vec![];

    let _r = html5ever::serialize(&mut page_bytes, &sh, SerializeOpts::default())?;
    Ok(page_bytes)
}

// /// Create `super::Page` from json.
// pub fn page_from_json2(
//     stor_root: &str,
//     page_path: &str,
//     page_json: &json::JsonValue,
// ) -> super::Page {
//     let source = source_from_json(page_path, page_json); // bytes

//     let mut page = super::Page::new(stor_root, page_path);
//     page.source.replace(Some(source));

//     page
// }

pub fn json_rev_match(page: &mut super::Page, json_data2: &json::JsonValue) -> Result<(), String> {
    if page.json().is_none() {
        return Err(format!("Failed to get json of {}", page.page_path));
    }

    let rev = match page.json().unwrap().rev() {
        Some(rev) => rev,
        None => return Err(format!("Failed to get rev of {}", page.page_path)),
    };

    let rev2 = match json_data2["data"]["page"]["rev"] {
        json::JsonValue::Number(number) => number.try_into().or(Err(())),
        // case: rev="12" ( with "" )
        json::JsonValue::Short(short) => {
            let rev2 = short.as_str();
            usize::from_str(rev2).or(Err(()))
        }
        _ => Err(()),
    }
    .or(Err(format!("Failed to get rev from json_data2")))?;

    if rev == rev2 {
        Ok(())
    } else {
        Err(format!("rev not match {} : {}", rev, rev2))
    }
}

/// Create a new page under the parent_page
/// It returns an instance of super::Page
/// but its file is not saved.
/// You need to save the file if needs.
///
/// It return Err if a file alraady exists in the child_href,
///
/// Create new navi data taking over parent_page and adding child_title to it
/// converting href based on child_href
///
/// parent_* : some value of parent_page
/// child_* : some value of new navi data created in this function
///
/// child_href: absolute or related location based on parent_page
///
pub fn page_child_new(
    parent_page: &mut super::Page,
    child_title: &str,
    child_href: &str,
) -> Result<super::Page, ()> {
    // If no parent json, no file or no data, return Err(())
    let _parent_json = parent_page.json().ok_or(())?;

    let parent_url = page_url(parent_page).or(Err(()))?;

    let (child_title, child_href) = title_href_check(child_title, child_href)?;

    // let child_url = url_on(&parent_url, child_href).or(Err(()))?;
    let child_url = parent_url.join(child_href).or_else(|_| {
        eprintln!("parent_url.join failed");
        Err(())
    })?;

    let child_path = child_url.path();

    // child_href might be a relative: ex: ./move2/move2.html, not for Page::new()
    // child_url.path(): /Computing/move2/move2.html
    let mut child_page_crt = super::Page::new(&parent_page.stor_root, child_path);

    // If the file already exists, return Err(())
    if child_page_crt.source().is_some() {
        info!("file {} already exists", child_page_crt.file_path());
        return Err(());
    }

    // json plain
    // let mut child_json = super::page_json::page_json_plain();
    let mut child_json = page_json::page_json_plain();

    // title
    child_json["data"]["page"]["title"] = child_title.into();

    // navi
    // navi of parent_page
    let mut child_navi = child_navi(parent_page, &parent_url, &child_url).or(Err(()))?;

    // add navi of child_href
    let navi_child: Vec<json::JsonValue> = vec![child_title.into(), "".into()];
    if child_navi.push(json::JsonValue::Array(navi_child)).is_err() {
        return Err(());
    }

    child_json["data"]["navi"] = child_navi;

    // let child_page = page_from_json(parent_page.stor_root(), child_path, &child_json);
    let child_page = Page::from_json(parent_page.stor_root(), child_path, &child_json);
    Ok(child_page)
}

/// Check title and href
fn title_href_check<'a>(title: &'a str, href: &'a str) -> Result<(&'a str, &'a str), ()> {
    let title = title.trim();
    if title.len() == 0 {
        eprintln!("no child title");
        return Err(());
    }

    let href = href.trim();
    if href.starts_with("#") {
        eprintln!("child href starts with #");
        return Err(());
    }
    if href.len() == 0 {
        eprintln!("no child href");
        return Err(());
    }

    Ok((title, href))
}

/// Create a navi data from parent_page except child_url and its title.
/// Convert href based on child_url as relative if possible.
fn child_navi(
    parent_page: &mut super::Page,
    parent_url: &url::Url,
    child_url: &url::Url,
) -> Result<json::JsonValue, ()> {
    let parent_json = parent_page.json().ok_or(())?;
    let parent_json = parent_json.value().ok_or(())?;

    let parent_navi = match &parent_json["data"]["navi"] {
        json::JsonValue::Array(ref v) => v,
        _ => return Err(()),
    };

    let mut child_navi = json::JsonValue::Array(vec![]);

    for navi in parent_navi {
        let title = navi[0].clone();

        // Convert href switching its base on paretn_url to child_url
        let href = navi[1]
            .as_str()
            .and_then(|href| href_url(&parent_url, href, &child_url))
            .or(Some("".to_string())) // only Some
            .unwrap();

        let mut navi2 = json::JsonValue::Array(vec![]);
        navi2.push::<json::JsonValue>(title.into()).or(Err(()))?;
        navi2.push::<json::JsonValue>(href.into()).or(Err(()))?;

        child_navi.push(navi2).or(Err(()))?;
    }

    Ok(child_navi)
}

/// Convert href to href_url based on org_base.
/// And get relative url of href based on new_base if posibble.
fn href_url(org_base: &url::Url, href: &str, new_base: &url::Url) -> Option<String> {
    // Get Url of href based on org_base
    match org_base.join(&href) {
        // Get relative url of href_url based on new_base
        Ok(href_url) => match new_base.make_relative(&href_url) {
            Some(v) => Some(v),
            // No relative exists, so absolute url of href_url(href)
            None => Some(href_url.as_str().to_string()),
        },
        Err(_) => None,
    }
}

/// Check relation between base_url and org_href and return a href
/// based on base_url
/// and wethere the href is a link to a child of urg_url.
/// Returns Option<(String, bool)
///  String: href, bool: true: is child.
///
/// If org_href is a link to base_url.path(), a href becomes as "#abc".
/// If org_href is a link to a child of base_url, a href becomes a relative value like a "child/child.html".
/// If org_href is a link not base_url.path() or its children,
/// a href becomes an absolute value like "/abc/def/ght.html" (start with /).
/// This absolute rule is essential of this system.
/// If org_href is not same host, returns org_href.
///
/// If a page moves to a different path, still relative href can work,
/// and absolute href as well, so the destination url is not concerned.
///
fn href_on(base_url: &url::Url, org_href: &str) -> Option<(String, bool)> {
    let org_href_url = base_url.join(org_href).ok()?;

    let is_not_child = false;

    // Case the host is not of this page, return the original value as it is.
    if org_href_url.host() != base_url.host() {
        // full url
        let href = org_href_url.as_str().to_string();
        return Some((href, is_not_child));
    }

    // Case org_href path is as same as base_url path, means same page.
    // if org_href is empty, no need to make a new link.
    if base_url.path() == org_href_url.path() {
        // org_href may be as same as href we get here,
        // but org_href might have some more infomation than the reference.
        // fragment: (#)subsection1 (exclude #)
        let fragment = org_href_url.fragment()?;
        let href = "#".to_string() + fragment;

        return Some((href, is_not_child));
    }

    // Case org_href is child of base_url,
    // In case org_href path is as same as base_url path, it was handled previously and it does not come here.
    // relative href can be used, so you can forget about dest_url
    //
    // remove file name from base_url.path()
    // base_url: "http://127.0.0.1/path/filename"
    // path_secment:  path, filename
    // last: filename
    let filename = base_url.path_segments().and_then(|split| split.last())?;
    let org_dir = base_url.path().strip_suffix(filename)?;
    if org_href_url.path().starts_with(org_dir) {
        //      org_dir: org/url/  (base_url without filename)
        // org_href_url: org/url/href/page.html#fragment
        // remove prefix(: org/url/ ), remains: href/page.html
        // href: href/page.html
        let mut href = org_href_url.path().strip_prefix(org_dir)?.to_string();
        if let Some(fragment) = org_href_url.fragment() {
            href = href + "#" + fragment;
        }
        // dest base
        let is_child = true;
        return Some((href, is_child));
    }

    // Case not child of the orig_url
    let org_href_url = base_url.join(org_href).unwrap();
    let dest_href_url = org_href_url;
    let mut href = dest_href_url.path().to_string();
    if let Some(fragment) = dest_href_url.fragment() {
        href = href + "#" + fragment;
    }

    Some((href, is_not_child))
}

fn page_mainte(
    page: &mut super::Page,
    recursive: bool,
    log: Option<Rc<RefCell<page_mainte::page_form_update::Log>>>,
) {
    page_mainte::page_mainte(page, recursive, log)
}

/// Upgrade old page type.
pub fn page_upgrade(page: &mut super::Page, upres: Option<Rc<RefCell<Upres>>>) {
    return page_upgrade::page_upgrade(page, upres);
}

// upgrade_and_backup_delete

pub fn page_upgrade_and_delete_children(
    page: &mut super::Page,
    recursive: bool,
    upres: Option<Rc<RefCell<Upres>>>,
) {
    // info!("fn page_upgrade_children");

    // To avoid
    // error[E0499]: cannot borrow `*page` as mutable more than once at a time
    // get page_url at here previously
    let Ok(page_url) = page_url(page) else {
        return;
    };

    let stor_root = page.stor_root().to_string();

    // let page_json = page.json();
    let page_json = page.json_mut();
    if page_json.is_none() {
        return;
    }
    let Some(subsections_json) = page_json.unwrap().subsections() else {
        return;
    };

    for (_, subsection_json) in subsections_json.iter() {
        // subsection_json["href"]
        // info!("href: {}", subsection_json["href"]);

        let Some(href) = subsection_json["href"].as_str() else {
            continue;
        };

        // href is not to child of the page
        let Some((_, is_child)) = href_on(&page_url, href) else {
            continue;
        };
        if !is_child {
            continue;
        }

        let Ok(href_url) = page_url.join(href) else {
            continue;
        };

        // info!("href_url: {}", href_url);

        let mut child_page = super::Page::new(&stor_root, href_url.path());
        let upres_child = upres.as_ref().and_then(|ref v| Some(Rc::clone(v)));
        // child_page.upgrade(recursive, upres_child);
        child_page.upgrade_and_backup_delete(recursive, upres_child);
    }
}

// pub fn page_upgrade_children(
//     page: &mut super::Page,
//     recursive: bool,
//     upres: Option<Rc<RefCell<Upres>>>,
// ) {
//     // info!("fn page_upgrade_children");

//     // To avoid
//     // error[E0499]: cannot borrow `*page` as mutable more than once at a time
//     // get page_url at here previously
//     let Ok(page_url) = page_url(page) else {
//         return;
//     };

//     let stor_root = page.stor_root().to_string();

//     // let page_json = page.json();
//     let page_json = page.json_mut();
//     if page_json.is_none() {
//         return;
//     }
//     let Some(subsections_json) = page_json.unwrap().subsections() else {
//         return;
//     };

//     for (_, subsection_json) in subsections_json.iter() {
//         // subsection_json["href"]
//         info!("href: {}", subsection_json["href"]);

//         let Some(href) = subsection_json["href"].as_str() else {
//             continue;
//         };

//         // href is not to child of the page
//         let Some((_, is_child)) = href_on(&page_url, href) else {
//             continue;
//         };
//         if !is_child {
//             continue;
//         }

//         let Ok(href_url) = page_url.join(href) else {
//             continue;
//         };

//         // info!("href_url: {}", href_url);

//         let mut child_page = super::Page::new(&stor_root, href_url.path());
//         let upres_child = upres.as_ref().and_then(|ref v| Some(Rc::clone(v)));
//         child_page.upgrade(recursive, upres_child);
//     }
// }

// pub fn page_upgrade_children_(
//     page: &mut super::Page,
//     recursive: bool,
//     upres: Option<Rc<RefCell<Upres>>>,
// ) {
//     // info!("fn page_upgrade_children");

//     // To avoid
//     // error[E0499]: cannot borrow `*page` as mutable more than once at a time
//     // get page_url at here previously
//     let Ok(page_url) = page_url(page) else {
//         return;
//     };

//     let stor_root = page.stor_root().to_string();

//     // let page_json = page.json();
//     let page_json = page.json_mut();
//     if page_json.is_none() {
//         return;
//     }
//     let Some(subsections_json) = page_json.unwrap().subsections() else {
//         return;
//     };

//     let subsection_top_json = &subsections_json["0"];
//     if subsection_top_json.is_null() {
//         return;
//     }
//     let children_id_json = match subsection_top_json["child"] {
//         json::JsonValue::Array(ref v) => v,
//         _ => return,
//     };

//     for id in children_id_json {
//         let subsection_json = &subsections_json[id.to_string().as_str()];
//         let Some(href) = subsection_json["href"].as_str() else {
//             continue;
//         };

//         // href is not to child of the page
//         let Some((_, is_child)) = href_on(&page_url, href) else {
//             continue;
//         };
//         if !is_child {
//             continue;
//         }

//         let Ok(href_url) = page_url.join(href) else {
//             continue;
//         };

//         let mut child_page = super::Page::new(&stor_root, href_url.path());
//         let upres_child = upres.as_ref().and_then(|ref v| Some(Rc::clone(v)));
//         child_page.upgrade(recursive, upres_child);
//     }
// }
