use html5ever::serialize::SerializeOpts;
use markup5ever_rcdom::{Handle, NodeData, RcDom, SerializableHandle}; // , Node
                                                                      // use html5ever::{parse_document, serialize};
use html5ever::serialize;
// use markup5ever_rcdom::{Handle, NodeData, RcDom, SerializableHandle}; // , Node
mod dom_utility;
// use std::io::{Error, ErrorKind};

// mod dom_utility;

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

fn page_dom_template() -> RcDom {
    to_dom(page_html_template())
}

fn page_html_template() -> &'static str {
    r#"<!DOCTYPE html><html><head><title></title><meta charset="UTF-8"></meta><script src="/wc.js"></script>
    <link rel="stylesheet" href="/wc.css"></link>
    <style type="text/css"></style>
</head><body onload="bodyOnload()"><span id="page_json_str" style="display: none"></span></body></html>
"#
}

pub fn page_from_json(
    page_json: json::JsonValue,
    path: &str,
    // ) -> Result<super::Page, std::io::Error> {
) -> Result<super::Page, ()> {
    let page_dom = page_dom_template();

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
    let mut bytes = vec![];
    let _r = serialize(&mut bytes, &sh, SerializeOpts::default());

    let mut page = match super::Page::from_path(path) {
        Ok(p) => p,
        // Err(e) => return Err(e),
        Err(_) => return Err(()),
    };

    page.source.replace(bytes);

    Ok(page)
}
