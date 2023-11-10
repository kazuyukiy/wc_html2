use html5ever::serialize;
use html5ever::serialize::SerializeOpts;
use markup5ever_rcdom::{Handle, NodeData, RcDom, SerializableHandle}; // , Node
use std::fs;

mod dom_utility;

pub fn to_dom(source: &str) -> RcDom {
    dom_utility::to_dom(source)
}

// Get span node from page_dom
// <span id="page_json_str" style="display: none">{"json":"json_data"}</span>
pub fn span_json_node(page_dom: &RcDom) -> Handle {
    let attrs = &vec![("id", "page_json_str")];
    let ptn_span = dom_utility::node_element("span", &attrs);
    // let ptn_span = span_json_node_ptn();
    // the <span> mut exists
    dom_utility::child_match_first(&page_dom, &ptn_span, true).unwrap()
}

pub fn json_from_dom(dom: &RcDom) -> Option<json::JsonValue> {
    // span node containing json data in text
    let span = span_json_node(dom);
    let children = span.children.borrow();
    if children.len() == 0 {
        eprintln!("Failed, json contents not found in the span element");
        return None;
    }

    let contents = match &children[0].data {
        NodeData::Text { contents } => contents,
        // _ => return Err(err_no_json_value()),
        _ => {
            eprintln!("Failed, json contents not found in the span element");
            return None;
        }
    };

    let json_str = contents.borrow().to_string();

    let json_value = match json::parse(&json_str) {
        Ok(page_json_parse) => page_json_parse,
        Err(e) => {
            eprintln!("{:?}", e);
            return None;
        }
    };

    Some(json_value)
}

fn page_dom_plain() -> RcDom {
    to_dom(page_html_plain())
}

fn page_html_plain() -> &'static str {
    r#"<!DOCTYPE html><html><head><title></title><meta charset="UTF-8"></meta><script src="/wc.js"></script>
    <link rel="stylesheet" href="/wc.css"></link>
    <style type="text/css"></style>
</head><body onload="bodyOnload()"><span id="page_json_str" style="display: none"></span></body></html>
"#
}

/// Create a page source from json value.
fn source_from_json(page_json: json::JsonValue) -> Vec<u8> {
    let page_dom = page_dom_plain();

    // title
    if let Some(title_str) = page_json["data"]["page"]["title"].as_str() {
        let title_ptn = dom_utility::node_element("title", &vec![]);
        if let Some(title_node) = dom_utility::child_match_first(&page_dom, &title_ptn, true) {
            let title_text = dom_utility::node_text(title_str);
            title_node.children.borrow_mut().push(title_text);
        }
    }

    // put json value into span as str
    let span = span_json_node(&page_dom);
    let _ = &span.children.borrow_mut().clear();
    let json_str = json::stringify(page_json);
    let json_node_text = dom_utility::node_text(&json_str);
    let _ = &span.children.borrow_mut().push(json_node_text);

    //
    let sh = SerializableHandle::from(page_dom.document);
    let mut page_bytes = vec![];
    let _r = serialize(&mut page_bytes, &sh, SerializeOpts::default());

    page_bytes
}

/// Create `supser::Page` from page_json and return it.
/// // If a file exists, open the file and overwrite page_json.
/// // If no file exists, create a `super::Page` from page_json.
///
pub fn page_from_json(
    page_root: &str,
    path: &str,
    page_json: json::JsonValue,
) -> Result<super::Page, ()> {
    let page_bytes = source_from_json(page_json);

    let mut page = match super::Page::new(page_root, path) {
        Ok(v) => v,
        Err(_) => match super::Page::open(page_root, path) {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Failed to get `Page`.");
                return Err(());
            }
        },
    };

    page.source.replace(page_bytes);

    Ok(page)
}

fn json_plain() -> json::JsonValue {
    // ~/projects/wc/wc/src/page_json_utility.rs
    json::object! {
        // "syttem" :
        "system" : {
            // "version" : "0.0.1",
            // "version" : "0.0.2",
            "version" : "0.0.3",
        },

        "data" : {
            "page" : {
                "title" : "",
                "rev" : 0,
                "rev_speculation" : 0,
                "group_top" : false,
        // consider to add path , uri data
            },

            "navi" : [
                /*
                {"name0" : "href0"},
                {"name1" : "href1"}
                // change to
                ["name0" , "href0"],
                ["name0" , "href0"],

                 */
            ],

            "subsection" : {
                "id" : {
                    "id_next" : 2,
                    "id_notinuse" : []
                },

                "data" : {
                    "0" : {
                        "parent" : "",
                        "id" : "0",
                        "title" : "",
                        "href" : "",
                        "content" : [],
                        "child" : []
                    }
                    // ,

                    // "1" : {
                        // "parent" : "0",
                        // "id" : "1",
                        // "title" : "sample",
                        // "href" : "#sample",
                        // "content" : [ {"type" : "text", "value" : "sample"} ],
                        // "child" : []
                    // }

                },

            },

            "href" : {
                "relation" : {},
                "last" : {},
                "history" : {},
            },
        },
    }
}

/// Create a new page under the parent_page
/// It returns an instance of super::Page
/// but its file is not saved.
/// You need to save the file if need it .
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
pub fn page_sub_new(
    parent_page: &super::Page,
    child_title: &str,
    child_href: &str,
) -> Result<super::Page, ()> {
    let (child_title, child_href) = child_title_href(child_title, child_href)?;
    let (parent_url, child_url) = parent_child_url(parent_page, child_href)?;

    let child_file_path = parent_page.file_path(child_url.path());
    if let Ok(_) = fs::File::open(&child_file_path) {
        // already the file exists.
        return Err(());
    }

    let parent_json = match parent_page.json() {
        Some(v) => v,
        None => return Err(()),
    };

    // Create child_json
    let mut child_json = json_plain();
    // title
    child_json["data"]["page"]["title"] = child_title.into();

    // navi
    let mut child_navi = child_navi_set(parent_url, &parent_json, &child_url)?;
    // add child page title as last navi element
    let navi_self: Vec<json::JsonValue> = vec![child_title.into(), "".into()];
    let _res = child_navi.push(json::JsonValue::Array(navi_self));
    child_json["data"]["navi"] = child_navi;

    // return Ok(Page)
    page_from_json(parent_page.page_root(), child_url.path(), child_json)
}

// Check child title and href
fn child_title_href<'a>(
    child_title: &'a str,
    child_href: &'a str,
) -> Result<(&'a str, &'a str), ()> {
    let child_title = child_title.trim();
    if child_title.len() == 0 {
        eprintln!("no child title");
        return Err(());
    }

    let child_href = child_href.trim();
    if child_href.starts_with("#") {
        eprintln!("child href starts with #");
        return Err(());
    }
    if child_href.len() == 0 {
        eprintln!("no child href");
        return Err(());
    }

    Ok((child_title, child_href))
}

fn parent_child_url<'a>(
    parent_page: &'a super::Page,
    child_href: &str,
) -> Result<(&'a url::Url, url::Url), ()> {
    let parent_url = match parent_page.url() {
        Some(v) => v,
        None => {
            eprintln!("no parent url");
            return Err(());
        }
    };

    // parent_url;
    let child_url = match parent_url.join(&child_href) {
        Ok(u) => u,
        Err(_) => {
            eprintln!("parent_url.join failed");
            return Err(());
        }
    };

    Ok((parent_url, child_url))
}

fn child_navi_set(
    parent_url: &url::Url,
    parent_json: &json::JsonValue,
    child_url: &url::Url,
) -> Result<json::JsonValue, ()> {
    let parent_navi = match &parent_json["data"]["navi"] {
        json::JsonValue::Array(ref v) => v,
        _ => return Err(()),
    };

    let mut child_navi = json::JsonValue::new_array();

    for navi in parent_navi {
        let title = match navi[0].as_str() {
            Some(v) => v,
            None => "no title",
        };

        // Convert href switching its base on paretn_url to child_url
        let href = match navi[1].as_str() {
            Some(parent_href) => match href_base_switch(&parent_url, parent_href, &child_url) {
                Some(v) => v,
                None => "".to_string(),
            },
            None => "".to_string(),
        };

        let navi2: Vec<json::JsonValue> = vec![title.into(), href.into()];
        let _ = child_navi.push(json::JsonValue::Array(navi2));
    }

    Ok(child_navi)
}

/// Return relative href of fm_href switching its base from fm_url to to_url.
fn href_base_switch(fm_url: &url::Url, fm_href: &str, to_url: &url::Url) -> Option<String> {
    // join : Get location of fm_href based on fm_url.
    match fm_url.join(&fm_href) {
        Ok(href_url) => {
            // make_relative: Get relative location of href_url based on to_Url.
            match to_url.make_relative(&href_url) {
                Some(v) => Some(v),
                // absolute
                None => Some(href_url.path().to_string()),
            }
        }
        Err(_) => None,
    }
}
