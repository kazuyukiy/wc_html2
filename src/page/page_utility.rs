// since I can not find out a way to use markup5ever_rcdom::RcDom in struct of Page under use hyper::service, use page_urility out of Page methods.
// error[E0277]: the trait bound `hyper::common::exec::Exec: hyper::common::exec::ConnStreamExec<impl Future<Output = Result<Response<Body>, hyper::Error>>, Body>` is not satisfied
//

use html5ever::serialize::SerializeOpts;
// use html5ever::{parse_document, serialize};
use html5ever::serialize;
use markup5ever_rcdom::{Handle, NodeData, RcDom, SerializableHandle}; // , Node
mod dom_utility;
use std::io::{Error, ErrorKind};
// use std::rc::Rc;

// this is only to show structure of page_json data
fn _page_json_style() {
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
    };
}

pub fn to_dom(source: &str) -> RcDom {
    dom_utility::to_dom(source)
}

// <span id="page_json_str" style="display: none">{"json":"json_data"}</span>
// fn span_json_node_ptn() -> Rc<Node> {
//     let attrs = &vec![("id", "page_json_str")];
//     dom_utility::node_element("span", &attrs)
// }

// Get span node from page_dom
// <span id="page_json_str" style="display: none">{"json":"json_data"}</span>
pub fn span_json_node(page_dom: &RcDom) -> Handle {
    let attrs = &vec![("id", "page_json_str")];
    let ptn_span = dom_utility::node_element("span", &attrs);
    // let ptn_span = span_json_node_ptn();
    // the <span> mut exists
    dom_utility::child_match_first(&page_dom, &ptn_span, true).unwrap()
}

pub fn json_from_dom(dom: &RcDom) -> Result<json::JsonValue, std::io::Error> {
    // span node containing json str as text
    let span = span_json_node(dom);
    let children = span.children.borrow();
    if children.len() == 0 {
        return Err(err_no_json_value());
    }
    // children: `Ref<'_, Vec<Rc<Node>>>`

    // children[0]
    // `Rc<Node>`

    let contents = match &children[0].data {
        NodeData::Text { contents } => contents,
        _ => return Err(err_no_json_value()),
    };

    let json_str = contents.borrow().to_string();

    let json_value = match json::parse(&json_str) {
        Ok(page_json_parse) => page_json_parse,
        Err(_) => {
            let err = Error::new(ErrorKind::Other, "Can not parse json value");
            return Err(err);
        }
    };

    Ok(json_value)
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
) -> Result<super::Page, std::io::Error> {
    let page_dom = page_dom_template();
    // [""][""][""]

    // title
    if let Some(title_str) = page_json["data"]["page"]["title"].as_str() {
        let title_ptn = dom_utility::node_element("title", &vec![]);
        if let Some(title_node) = dom_utility::child_match_first(&page_dom, &title_ptn, true) {
            let title_text = dom_utility::node_text(title_str);
            title_node.children.borrow_mut().push(title_text);
        }
    }

    // rev
    // if let Some(r) = page_json["data"]["page"]["rev"].as_u32() {
    // let rev_ptn =
    // self.rev.replace(*r);

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
        Err(e) => return Err(e),
    };

    page.source.replace(bytes);

    Ok(page)

    // let page_html = match String::from_utf8(bytes) {
    //     Ok(s) => s,
    //     Err(_) => return Err(Error::new(ErrorKind::Other, "Can not convert to UTF8")),
    // };

    // temp
    // Err(Error::new(ErrorKind::Other, "Can not convert to UTF8"))
}

// fn json_from_page(page: &mut super::Page) -> Result<json::JsonValue, std::io::Error> {
//     page.dom();

//     // temp
//     Err(Error::new(ErrorKind::Other, "Can not convert to UTF8"))
// }

fn err_no_json_value() -> std::io::Error {
    Error::new(ErrorKind::Other, "Can not finds span or json value")
}
// pub fn rev(dom: &RcDom) {}
